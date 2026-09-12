use crate::{
    qemu_command::{build_qemu_command_spec as build_spec, QemuCommandSpec},
    settings::{get_app_settings, AppSettings, CommandError},
    vm_config::VmConfiguration,
};
use serde::{Deserialize, Serialize};
use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
use tauri::AppHandle;

const VERSION_TIMEOUT: Duration = Duration::from_secs(2);
const QEMU_EXECUTABLE_NAMES: [&str; 2] = ["qemu-system-x86_64.exe", "qemu-system-x86_64"];

pub trait RuntimeAdapter {
    fn check_availability(&self) -> Result<RuntimeAvailability, CommandError>;
    fn discover_capabilities(&self, settings: &AppSettings) -> Result<RuntimeStatus, CommandError>;
    fn build_command_spec(
        &self,
        config: &VmConfiguration,
        status: &RuntimeStatus,
    ) -> Result<QemuCommandSpec, CommandError>;
    fn validate_vm_configuration(&self, config: &VmConfiguration) -> Result<(), CommandError>;
    fn create_vm_definition(
        &self,
        config: &VmConfiguration,
    ) -> Result<RuntimeVmDefinition, CommandError>;
    fn start_vm(&self, vm_id: &str) -> Result<(), CommandError>;
    fn stop_vm(&self, vm_id: &str) -> Result<(), CommandError>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeAvailability {
    pub available: bool,
    pub runtime: String,
    pub detail: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimeType {
    Qemu,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityState {
    Available,
    Unavailable,
    NotChecked,
    Unsupported,
    Error,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatus {
    pub state: CapabilityState,
    pub detail: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDiagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeStatus {
    pub runtime_type: RuntimeType,
    pub availability: CapabilityState,
    pub configured_qemu_path: Option<String>,
    pub detected_qemu_path: Option<String>,
    pub qemu_version: Option<String>,
    pub whpx: CapabilityStatus,
    pub virtualization: CapabilityStatus,
    pub diagnostics: Vec<RuntimeDiagnostic>,
    pub last_checked: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVmDefinition {
    pub id: String,
    pub name: String,
    pub summary: String,
}

#[derive(Default)]
pub struct QemuRuntimeAdapter;

impl RuntimeAdapter for QemuRuntimeAdapter {
    fn check_availability(&self) -> Result<RuntimeAvailability, CommandError> {
        Ok(RuntimeAvailability {
            available: false,
            runtime: "qemu".to_string(),
            detail: "QEMU runtime detection is not implemented yet.".to_string(),
        })
    }

    fn discover_capabilities(&self, settings: &AppSettings) -> Result<RuntimeStatus, CommandError> {
        discover_qemu_capabilities(settings)
    }

    fn build_command_spec(
        &self,
        config: &VmConfiguration,
        status: &RuntimeStatus,
    ) -> Result<QemuCommandSpec, CommandError> {
        build_spec(config, status)
    }

    fn validate_vm_configuration(&self, config: &VmConfiguration) -> Result<(), CommandError> {
        config.validate()
    }

    fn create_vm_definition(
        &self,
        config: &VmConfiguration,
    ) -> Result<RuntimeVmDefinition, CommandError> {
        self.validate_vm_configuration(config)?;
        Err(not_implemented(
            "Creating VM definitions is not implemented yet.",
        ))
    }

    fn start_vm(&self, _vm_id: &str) -> Result<(), CommandError> {
        Err(not_implemented("Starting VMs is not implemented yet."))
    }

    fn stop_vm(&self, _vm_id: &str) -> Result<(), CommandError> {
        Err(not_implemented("Stopping VMs is not implemented yet."))
    }
}

#[tauri::command]
pub fn get_runtime_status(app: AppHandle) -> Result<RuntimeStatus, CommandError> {
    refresh_runtime_status(app)
}

#[tauri::command]
pub fn refresh_runtime_status(app: AppHandle) -> Result<RuntimeStatus, CommandError> {
    let settings = get_app_settings(app)?;
    QemuRuntimeAdapter.discover_capabilities(&settings)
}

fn discover_qemu_capabilities(settings: &AppSettings) -> Result<RuntimeStatus, CommandError> {
    let mut diagnostics = Vec::new();
    let candidate = find_qemu_executable(settings, &mut diagnostics);
    let (availability, qemu_version) = match candidate.as_deref() {
        Some(path) => match detect_qemu_version(path) {
            Ok(version) => (CapabilityState::Available, Some(version)),
            Err(error) => {
                diagnostics.push(RuntimeDiagnostic {
                    code: "qemu_version_error".to_string(),
                    message: error,
                });
                (CapabilityState::Error, None)
            }
        },
        None => (CapabilityState::Unavailable, None),
    };

    let (whpx, virtualization) = platform_diagnostics();
    if candidate.is_none() {
        diagnostics.push(RuntimeDiagnostic {
            code: "qemu_not_found".to_string(),
            message: "QEMU was not detected. Configure a valid executable path or install QEMU in a standard location.".to_string(),
        });
    }

    Ok(RuntimeStatus {
        runtime_type: RuntimeType::Qemu,
        availability,
        configured_qemu_path: settings.qemu_executable_path.clone(),
        detected_qemu_path: candidate.map(|path| path.display().to_string()),
        qemu_version,
        whpx,
        virtualization,
        diagnostics,
        last_checked: current_timestamp(),
    })
}

fn find_qemu_executable(
    settings: &AppSettings,
    diagnostics: &mut Vec<RuntimeDiagnostic>,
) -> Option<PathBuf> {
    if let Some(configured_path) = settings.qemu_executable_path.as_deref() {
        let path = PathBuf::from(configured_path.trim());
        if is_executable_file(&path) {
            return Some(path);
        }
        diagnostics.push(RuntimeDiagnostic {
            code: "configured_qemu_path_invalid".to_string(),
            message: "The configured QEMU path is not a regular executable file.".to_string(),
        });
    }

    for path in standard_qemu_paths() {
        if is_executable_file(&path) {
            return Some(path);
        }
    }

    find_on_path()
}

fn standard_qemu_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for variable in ["ProgramFiles", "ProgramFiles(x86)"] {
        if let Some(root) = env::var_os(variable) {
            for name in QEMU_EXECUTABLE_NAMES {
                paths.push(PathBuf::from(&root).join("qemu").join(name));
            }
        }
    }
    paths
}

fn find_on_path() -> Option<PathBuf> {
    let path_variable = env::var_os("PATH")?;
    for directory in env::split_paths(&path_variable) {
        for name in QEMU_EXECUTABLE_NAMES {
            let candidate = directory.join(name);
            if is_executable_file(&candidate) {
                return Some(candidate);
            }
        }
    }
    None
}

fn is_executable_file(path: &Path) -> bool {
    path.is_file()
}

fn detect_qemu_version(path: &Path) -> Result<String, String> {
    let mut child = Command::new(path)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| "QEMU was found, but its version could not be queried.".to_string())?;
    let started = Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if started.elapsed() >= VERSION_TIMEOUT => {
                let _ = child.kill();
                return Err("QEMU version detection timed out.".to_string());
            }
            Ok(None) => thread::sleep(Duration::from_millis(10)),
            Err(_) => {
                let _ = child.kill();
                return Err("QEMU version detection could not be completed.".to_string());
            }
        }
    }

    let output = child
        .wait_with_output()
        .map_err(|_| "QEMU version output could not be read.".to_string())?;
    if !output.status.success() {
        return Err("QEMU rejected the version query.".to_string());
    }
    let text = String::from_utf8_lossy(&output.stdout);
    parse_qemu_version(&text).ok_or_else(|| "QEMU returned an unrecognized version.".to_string())
}

fn parse_qemu_version(output: &str) -> Option<String> {
    output
        .lines()
        .find(|line| line.to_ascii_lowercase().contains("qemu"))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

#[cfg(windows)]
fn platform_diagnostics() -> (CapabilityStatus, CapabilityStatus) {
    (
        CapabilityStatus {
            state: CapabilityState::Unknown,
            detail:
                "WHPX availability requires host capability detection that is not implemented yet."
                    .to_string(),
        },
        CapabilityStatus {
            state: CapabilityState::Unknown,
            detail: "CPU virtualization support could not be determined safely by this milestone."
                .to_string(),
        },
    )
}

#[cfg(not(windows))]
fn platform_diagnostics() -> (CapabilityStatus, CapabilityStatus) {
    let unsupported = CapabilityStatus {
        state: CapabilityState::Unsupported,
        detail: "WHPX diagnostics are supported only on Windows.".to_string(),
    };
    (
        unsupported.clone(),
        CapabilityStatus {
            state: CapabilityState::Unsupported,
            detail: "Windows virtualization diagnostics are supported only on Windows.".to_string(),
        },
    )
}

fn current_timestamp() -> String {
    format!(
        "{}s since UNIX epoch",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    )
}

#[tauri::command]
pub fn build_qemu_command_spec(
    app: AppHandle,
    vm_id: String,
) -> Result<QemuCommandSpec, CommandError> {
    let definition = crate::vm_definitions::get_vm_definition(app.clone(), vm_id)?;
    let settings = get_app_settings(app)?;
    let status = QemuRuntimeAdapter.discover_capabilities(&settings)?;
    QemuRuntimeAdapter.build_command_spec(&definition.configuration, &status)
}

fn not_implemented(message: impl Into<String>) -> CommandError {
    CommandError {
        code: "not_implemented",
        message: message.into(),
        field: None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        detect_qemu_version, parse_qemu_version, CapabilityState, QemuRuntimeAdapter,
        RuntimeAdapter,
    };
    use crate::{settings::AppSettings, vm_config::VmConfiguration};
    use std::path::Path;

    #[test]
    fn runtime_status_serializes_with_typed_states() {
        let status = QemuRuntimeAdapter
            .discover_capabilities(&AppSettings::default())
            .unwrap();
        let serialized = serde_json::to_string(&status).unwrap();
        assert!(serialized.contains("runtimeType"));
        assert!(matches!(
            status.availability,
            CapabilityState::Available | CapabilityState::Unavailable | CapabilityState::Error
        ));
    }

    #[test]
    fn invalid_configured_path_reports_unavailable_without_panicking() {
        let settings = AppSettings {
            qemu_executable_path: Some("C:\\definitely\\missing\\qemu.exe".to_string()),
            ..AppSettings::default()
        };
        let status = QemuRuntimeAdapter.discover_capabilities(&settings).unwrap();
        assert_eq!(status.availability, CapabilityState::Unavailable);
        assert!(status
            .diagnostics
            .iter()
            .any(|item| item.code == "configured_qemu_path_invalid"));
    }

    #[test]
    fn version_parser_extracts_qemu_line() {
        assert_eq!(
            parse_qemu_version("QEMU emulator version 9.0.0\nCopyright"),
            Some("QEMU emulator version 9.0.0".to_string())
        );
    }

    #[test]
    fn missing_executable_returns_structured_version_error() {
        assert!(detect_qemu_version(Path::new("missing-qemu-executable")).is_err());
    }

    #[test]
    fn placeholder_lifecycle_methods_remain_not_implemented() {
        let adapter = QemuRuntimeAdapter;
        assert_eq!(
            adapter.start_vm("vm-test").unwrap_err().code,
            "not_implemented"
        );
        assert_eq!(
            adapter.stop_vm("vm-test").unwrap_err().code,
            "not_implemented"
        );
        let configuration: VmConfiguration = serde_json::from_str(
            r#"{
                "id":"vm-test","name":"Test","operatingSystem":"linux","cpuCount":2,
                "memoryMiB":4096,"diskSizeGiB":64,"isoPath":"","networkMode":"user",
                "displayMode":"windowed","secureBootEnabled":false,"tpmEnabled":false
            }"#,
        )
        .unwrap();
        assert_eq!(
            adapter
                .create_vm_definition(&configuration)
                .unwrap_err()
                .code,
            "not_implemented"
        );
    }
}
