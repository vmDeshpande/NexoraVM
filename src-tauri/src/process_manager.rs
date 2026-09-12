use crate::{
    qemu_command::{validate_qemu_command_spec, QemuCommandSpec},
    runtime_adapter::{QemuRuntimeAdapter, RuntimeAdapter},
    settings::{get_app_settings, CommandError},
    vm_definitions::get_vm_definition,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{self, Read},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, State};

const MAX_OUTPUT_BYTES: usize = 64 * 1024;
const GRACEFUL_STOP_TIMEOUT: Duration = Duration::from_millis(500);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum QemuProcessState {
    NotStarted,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
    TimedOut,
    Cancelled,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum QemuTerminationReason {
    Graceful,
    Forced,
    UnexpectedExit,
    FailedToStart,
    TimedOut,
    Cancelled,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QemuProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub truncated: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QemuProcessStatus {
    pub vm_id: String,
    pub state: QemuProcessState,
    pub process_id: Option<u32>,
    pub exit_code: Option<i32>,
    pub termination_reason: Option<QemuTerminationReason>,
    pub output: QemuProcessOutput,
}

pub struct ProcessManagerState {
    manager: Mutex<QemuProcessManager<ActualProcessLauncher>>,
}

impl Default for ProcessManagerState {
    fn default() -> Self {
        Self {
            manager: Mutex::new(QemuProcessManager::default()),
        }
    }
}

trait ManagedChild: Send {
    fn process_id(&self) -> u32;
    fn try_wait(&mut self) -> io::Result<Option<i32>>;
    fn kill(&mut self) -> io::Result<()>;
    fn request_graceful_stop(&mut self) -> io::Result<bool>;
    fn output(&self) -> QemuProcessOutput;
}

trait ProcessLauncher: Default + Send {
    fn launch(&self, spec: &QemuCommandSpec) -> io::Result<Box<dyn ManagedChild>>;
}

#[derive(Default)]
struct ActualProcessLauncher;

impl ProcessLauncher for ActualProcessLauncher {
    fn launch(&self, spec: &QemuCommandSpec) -> io::Result<Box<dyn ManagedChild>> {
        let mut command = Command::new(&spec.executable_path);
        command
            .args(&spec.arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(directory) = &spec.working_directory {
            command.current_dir(directory);
        }

        let mut child = command.spawn()?;
        let stdout = child.stdout.take().map(start_bounded_reader);
        let stderr = child.stderr.take().map(start_bounded_reader);
        Ok(Box::new(ChildProcess {
            child,
            stdout,
            stderr,
        }))
    }
}

struct ChildProcess {
    child: Child,
    stdout: Option<Arc<Mutex<BoundedBuffer>>>,
    stderr: Option<Arc<Mutex<BoundedBuffer>>>,
}

impl ManagedChild for ChildProcess {
    fn process_id(&self) -> u32 {
        self.child.id()
    }

    fn try_wait(&mut self) -> io::Result<Option<i32>> {
        Ok(self
            .child
            .try_wait()?
            .map(|status| status.code().unwrap_or(-1)))
    }

    fn kill(&mut self) -> io::Result<()> {
        self.child.kill()
    }

    fn request_graceful_stop(&mut self) -> io::Result<bool> {
        Ok(false)
    }

    fn output(&self) -> QemuProcessOutput {
        QemuProcessOutput {
            stdout: read_bounded_buffer(&self.stdout).0,
            stderr: read_bounded_buffer(&self.stderr).0,
            truncated: read_bounded_buffer(&self.stdout).1 || read_bounded_buffer(&self.stderr).1,
        }
    }
}

struct BoundedBuffer {
    bytes: Vec<u8>,
    truncated: bool,
}

fn start_bounded_reader<R: Read + Send + 'static>(mut reader: R) -> Arc<Mutex<BoundedBuffer>> {
    let buffer = Arc::new(Mutex::new(BoundedBuffer {
        bytes: Vec::new(),
        truncated: false,
    }));
    let target = Arc::clone(&buffer);
    thread::spawn(move || {
        let mut chunk = [0_u8; 4096];
        loop {
            let read = match reader.read(&mut chunk) {
                Ok(0) | Err(_) => break,
                Ok(read) => read,
            };
            let Ok(mut output) = target.lock() else {
                break;
            };
            let remaining = MAX_OUTPUT_BYTES.saturating_sub(output.bytes.len());
            if remaining > 0 {
                output
                    .bytes
                    .extend_from_slice(&chunk[..read.min(remaining)]);
            }
            if read > remaining {
                output.truncated = true;
            }
        }
    });
    buffer
}

struct ProcessEntry {
    status: QemuProcessStatus,
    child: Option<Box<dyn ManagedChild>>,
}

struct QemuProcessManager<L: ProcessLauncher = ActualProcessLauncher> {
    launcher: L,
    processes: HashMap<String, ProcessEntry>,
}

impl Default for QemuProcessManager<ActualProcessLauncher> {
    fn default() -> Self {
        Self {
            launcher: ActualProcessLauncher,
            processes: HashMap::new(),
        }
    }
}

impl<L: ProcessLauncher> QemuProcessManager<L> {
    #[cfg(test)]
    fn with_launcher(launcher: L) -> Self {
        Self {
            launcher,
            processes: HashMap::new(),
        }
    }

    fn start(&mut self, spec: QemuCommandSpec) -> Result<QemuProcessStatus, CommandError> {
        validate_qemu_command_spec(&spec)?;
        if let Some(entry) = self.processes.get(&spec.vm_id) {
            if matches!(
                entry.status.state,
                QemuProcessState::Starting | QemuProcessState::Running | QemuProcessState::Stopping
            ) {
                return Err(CommandError::conflict(
                    "A QEMU process is already active for this VM.",
                ));
            }
        }
        self.processes.remove(&spec.vm_id);

        let vm_id = spec.vm_id.clone();
        let child = self.launcher.launch(&spec).map_err(|error| {
            CommandError::runtime(format!("QEMU could not be started: {error}"))
        })?;
        let process_id = child.process_id();
        let mut entry = ProcessEntry {
            status: QemuProcessStatus {
                vm_id: vm_id.clone(),
                state: QemuProcessState::Starting,
                process_id: Some(process_id),
                exit_code: None,
                termination_reason: None,
                output: QemuProcessOutput::default(),
            },
            child: Some(child),
        };
        if let Some(active_child) = entry.child.as_mut() {
            match active_child.try_wait() {
                Ok(None) => entry.status.state = QemuProcessState::Running,
                Ok(Some(exit_code)) => {
                    entry.status.output = active_child.output();
                    entry.status.state = QemuProcessState::Failed;
                    entry.status.exit_code = Some(exit_code);
                    entry.status.termination_reason = Some(QemuTerminationReason::UnexpectedExit);
                    entry.child = None;
                }
                Err(error) => {
                    entry.status.output = active_child.output();
                    entry.status.state = QemuProcessState::Failed;
                    entry.status.termination_reason = Some(QemuTerminationReason::FailedToStart);
                    entry.child = None;
                    return Err(CommandError::runtime(format!(
                        "QEMU process status could not be checked: {error}"
                    )));
                }
            }
        }
        let status = entry.status.clone();
        self.processes.insert(vm_id, entry);
        if status.state == QemuProcessState::Failed {
            return Err(CommandError::runtime(
                "QEMU exited before it could become running.",
            ));
        }
        Ok(status)
    }

    fn status(&mut self, vm_id: &str) -> Result<QemuProcessStatus, CommandError> {
        let Some(entry) = self.processes.get_mut(vm_id) else {
            return Ok(QemuProcessStatus {
                vm_id: vm_id.to_string(),
                state: QemuProcessState::NotStarted,
                process_id: None,
                exit_code: None,
                termination_reason: None,
                output: QemuProcessOutput::default(),
            });
        };
        refresh_entry(entry);
        Ok(snapshot_entry(entry))
    }

    fn stop(&mut self, vm_id: &str) -> Result<QemuProcessStatus, CommandError> {
        let Some(entry) = self.processes.get_mut(vm_id) else {
            return Ok(QemuProcessStatus {
                vm_id: vm_id.to_string(),
                state: QemuProcessState::Stopped,
                process_id: None,
                exit_code: None,
                termination_reason: None,
                output: QemuProcessOutput::default(),
            });
        };
        refresh_entry(entry);
        if entry.child.is_none() {
            return Ok(snapshot_entry(entry));
        }

        entry.status.state = QemuProcessState::Stopping;
        let graceful = entry
            .child
            .as_mut()
            .map(|child| child.request_graceful_stop().unwrap_or(false))
            .unwrap_or(false);
        if graceful && wait_for_exit(entry, GRACEFUL_STOP_TIMEOUT) {
            entry.status.state = QemuProcessState::Stopped;
            entry.status.termination_reason = Some(QemuTerminationReason::Graceful);
        } else {
            let kill_result = entry
                .child
                .as_mut()
                .map(|child| child.kill())
                .unwrap_or_else(|| Err(io::Error::other("QEMU process handle is unavailable")));
            if kill_result.is_err() || !wait_for_exit(entry, GRACEFUL_STOP_TIMEOUT) {
                entry.status.state = QemuProcessState::TimedOut;
                entry.status.termination_reason = Some(QemuTerminationReason::TimedOut);
            } else {
                entry.status.state = QemuProcessState::Stopped;
                entry.status.termination_reason = Some(QemuTerminationReason::Forced);
            }
        }
        if let Some(child) = entry.child.as_ref() {
            entry.status.output = child.output();
        }
        entry.child = None;
        Ok(snapshot_entry(entry))
    }
}

fn refresh_entry(entry: &mut ProcessEntry) {
    if let Some(child) = entry.child.as_mut() {
        match child.try_wait() {
            Ok(None) => {
                if entry.status.state == QemuProcessState::Starting {
                    entry.status.state = QemuProcessState::Running;
                }
            }
            Ok(Some(exit_code)) => {
                entry.status.output = child.output();
                entry.status.exit_code = Some(exit_code);
                entry.status.state = if exit_code == 0 {
                    QemuProcessState::Stopped
                } else {
                    QemuProcessState::Failed
                };
                entry.status.termination_reason = Some(QemuTerminationReason::UnexpectedExit);
                entry.child = None;
            }
            Err(_) => {
                entry.status.output = child.output();
                entry.status.state = QemuProcessState::Failed;
                entry.status.termination_reason = Some(QemuTerminationReason::UnexpectedExit);
                entry.child = None;
            }
        }
    }
    if let Some(child) = entry.child.as_ref() {
        entry.status.output = child.output();
    }
}

fn wait_for_exit(entry: &mut ProcessEntry, timeout: Duration) -> bool {
    let started = Instant::now();
    while started.elapsed() < timeout {
        if let Some(child) = entry.child.as_mut() {
            if let Ok(Some(exit_code)) = child.try_wait() {
                entry.status.exit_code = Some(exit_code);
                return true;
            }
        } else {
            return true;
        }
        thread::sleep(Duration::from_millis(10));
    }
    false
}

fn read_bounded_buffer(buffer: &Option<Arc<Mutex<BoundedBuffer>>>) -> (String, bool) {
    buffer
        .as_ref()
        .and_then(|buffer| buffer.lock().ok())
        .map(|buffer| {
            (
                String::from_utf8_lossy(&buffer.bytes).into_owned(),
                buffer.truncated,
            )
        })
        .unwrap_or_default()
}

fn snapshot_entry(entry: &ProcessEntry) -> QemuProcessStatus {
    entry.status.clone()
}

#[tauri::command]
pub fn start_vm(
    app: AppHandle,
    state: State<'_, ProcessManagerState>,
    vm_id: String,
) -> Result<QemuProcessStatus, CommandError> {
    let definition = get_vm_definition(app.clone(), vm_id)?;
    let settings = get_app_settings(app)?;
    let adapter = QemuRuntimeAdapter;
    let runtime_status = adapter.discover_capabilities(&settings)?;
    let spec = adapter.build_command_spec(&definition.configuration, &runtime_status)?;
    state
        .manager
        .lock()
        .map_err(|_| CommandError::runtime("The QEMU process manager is unavailable."))?
        .start(spec)
}

#[tauri::command]
pub fn stop_vm(
    app: AppHandle,
    state: State<'_, ProcessManagerState>,
    vm_id: String,
) -> Result<QemuProcessStatus, CommandError> {
    get_vm_definition(app, vm_id.clone())?;
    state
        .manager
        .lock()
        .map_err(|_| CommandError::runtime("The QEMU process manager is unavailable."))?
        .stop(&vm_id)
}

#[tauri::command]
pub fn get_vm_process_status(
    app: AppHandle,
    state: State<'_, ProcessManagerState>,
    vm_id: String,
) -> Result<QemuProcessStatus, CommandError> {
    get_vm_definition(app, vm_id.clone())?;
    state
        .manager
        .lock()
        .map_err(|_| CommandError::runtime("The QEMU process manager is unavailable."))?
        .status(&vm_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{qemu_command::QemuCommandSpec, runtime_adapter::RuntimeDiagnostic};
    use std::path::PathBuf;

    #[derive(Default)]
    struct MockLauncher {
        launch_fails: bool,
        exits_immediately: bool,
        graceful_stop: bool,
        graceful_completes: bool,
    }

    struct MockChild {
        id: u32,
        alive: bool,
        graceful: bool,
        graceful_completes: bool,
        graceful_requested: bool,
    }

    impl ManagedChild for MockChild {
        fn process_id(&self) -> u32 {
            self.id
        }
        fn try_wait(&mut self) -> io::Result<Option<i32>> {
            Ok((!self.alive).then_some(0))
        }
        fn kill(&mut self) -> io::Result<()> {
            self.alive = false;
            Ok(())
        }
        fn request_graceful_stop(&mut self) -> io::Result<bool> {
            if self.graceful {
                self.graceful_requested = true;
                if self.graceful_completes {
                    self.alive = false;
                }
            }
            Ok(self.graceful)
        }

        fn output(&self) -> QemuProcessOutput {
            QemuProcessOutput::default()
        }
    }

    impl ProcessLauncher for MockLauncher {
        fn launch(&self, _spec: &QemuCommandSpec) -> io::Result<Box<dyn ManagedChild>> {
            if self.launch_fails {
                return Err(io::Error::other("mock launch failure"));
            }
            Ok(Box::new(MockChild {
                id: 42,
                alive: !self.exits_immediately,
                graceful: self.graceful_stop,
                graceful_completes: self.graceful_completes,
                graceful_requested: false,
            }))
        }
    }

    fn spec() -> QemuCommandSpec {
        QemuCommandSpec {
            executable_path: std::env::current_exe().unwrap(),
            arguments: vec!["--help".to_string()],
            working_directory: None,
            vm_id: "vm-test".to_string(),
            diagnostics: Vec::<RuntimeDiagnostic>::new(),
            acceleration: crate::qemu_command::QemuAcceleration::Tcg,
            display_mode: crate::qemu_command::QemuDisplayMode::None,
            network_mode: crate::qemu_command::QemuNetworkMode::None,
        }
    }

    #[test]
    fn start_transitions_to_running_and_duplicate_start_is_rejected() {
        let mut manager = QemuProcessManager::with_launcher(MockLauncher::default());
        assert_eq!(
            manager.start(spec()).unwrap().state,
            QemuProcessState::Running
        );
        assert_eq!(manager.start(spec()).unwrap_err().code, "conflict");
    }

    #[test]
    fn stop_transitions_to_stopped_and_cleans_handle() {
        let mut manager = QemuProcessManager::with_launcher(MockLauncher::default());
        manager.start(spec()).unwrap();
        let stopped = manager.stop("vm-test").unwrap();
        assert_eq!(stopped.state, QemuProcessState::Stopped);
        assert_eq!(
            manager.status("vm-test").unwrap().state,
            QemuProcessState::Stopped
        );
    }

    #[test]
    fn invalid_and_empty_specs_are_rejected_before_launch() {
        let mut manager = QemuProcessManager::with_launcher(MockLauncher::default());
        let mut invalid = spec();
        invalid.executable_path = PathBuf::new();
        assert_eq!(
            manager.start(invalid).unwrap_err().field,
            Some("executablePath")
        );
        let mut empty = spec();
        empty.arguments.clear();
        assert_eq!(manager.start(empty).unwrap_err().field, Some("arguments"));
    }

    #[test]
    fn output_is_bounded() {
        let mut buffer = BoundedBuffer {
            bytes: Vec::new(),
            truncated: false,
        };
        let data = vec![b'x'; MAX_OUTPUT_BYTES + 1];
        let remaining = MAX_OUTPUT_BYTES.saturating_sub(buffer.bytes.len());
        buffer
            .bytes
            .extend_from_slice(&data[..data.len().min(remaining)]);
        buffer.truncated = data.len() > remaining;
        assert_eq!(buffer.bytes.len(), MAX_OUTPUT_BYTES);
        assert!(buffer.truncated);
    }

    #[test]
    fn failed_launch_returns_structured_error() {
        let mut manager = QemuProcessManager::with_launcher(MockLauncher {
            launch_fails: true,
            ..MockLauncher::default()
        });
        let error = manager.start(spec()).unwrap_err();
        assert_eq!(error.code, "runtime_error");
    }

    #[test]
    fn immediate_exit_is_never_reported_as_running() {
        let mut manager = QemuProcessManager::with_launcher(MockLauncher {
            exits_immediately: true,
            ..MockLauncher::default()
        });
        let error = manager.start(spec()).unwrap_err();
        assert_eq!(error.code, "runtime_error");
        assert_eq!(
            manager.status("vm-test").unwrap().state,
            QemuProcessState::Failed
        );
    }

    #[test]
    fn graceful_stop_uses_graceful_termination_reason() {
        let mut manager = QemuProcessManager::with_launcher(MockLauncher {
            graceful_stop: true,
            graceful_completes: true,
            ..MockLauncher::default()
        });
        manager.start(spec()).unwrap();
        let status = manager.stop("vm-test").unwrap();
        assert_eq!(status.state, QemuProcessState::Stopped);
        assert_eq!(
            status.termination_reason,
            Some(QemuTerminationReason::Graceful)
        );
    }

    #[test]
    fn graceful_stop_timeout_falls_back_to_forced_termination() {
        let mut manager = QemuProcessManager::with_launcher(MockLauncher {
            graceful_stop: true,
            graceful_completes: false,
            ..MockLauncher::default()
        });
        manager.start(spec()).unwrap();
        let status = manager.stop("vm-test").unwrap();
        assert_eq!(status.state, QemuProcessState::Stopped);
        assert_eq!(
            status.termination_reason,
            Some(QemuTerminationReason::Forced)
        );
    }
}
