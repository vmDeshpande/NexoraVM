use crate::{
    runtime_adapter::{CapabilityState, RuntimeDiagnostic, RuntimeStatus},
    settings::{CommandError, DisplayMode},
    vm_config::{NetworkMode, VmConfiguration},
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum QemuAcceleration {
    Whpx,
    Tcg,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum QemuDisplayMode {
    Sdl,
    GtkFullscreen,
    None,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum QemuNetworkMode {
    None,
    User,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QemuCommandSpec {
    pub executable_path: PathBuf,
    pub arguments: Vec<String>,
    pub working_directory: Option<PathBuf>,
    pub vm_id: String,
    pub diagnostics: Vec<RuntimeDiagnostic>,
    pub acceleration: QemuAcceleration,
    pub display_mode: QemuDisplayMode,
    pub network_mode: QemuNetworkMode,
}

pub fn build_qemu_command_spec(
    config: &VmConfiguration,
    status: &RuntimeStatus,
) -> Result<QemuCommandSpec, CommandError> {
    validate_command_inputs(config, status)?;

    let executable_path = PathBuf::from(
        status
            .detected_qemu_path
            .as_deref()
            .ok_or_else(|| CommandError::runtime("No detected QEMU executable is available."))?,
    );
    let acceleration = select_acceleration(status);
    let display_mode = map_display_mode(&config.display_mode)?;
    let network_mode = map_network_mode(&config.network_mode)?;
    let mut arguments = vec![
        "-name".to_string(),
        config.name.trim().to_string(),
        "-machine".to_string(),
        "q35".to_string(),
        "-smp".to_string(),
        config.cpu_count.to_string(),
        "-m".to_string(),
        config.memory_mi_b.to_string(),
        "-accel".to_string(),
        acceleration_argument(&acceleration).to_string(),
        "-display".to_string(),
        display_argument(&display_mode).to_string(),
        "-nic".to_string(),
        network_argument(&network_mode).to_string(),
    ];

    if let Some(disk_path) = config.disk_path.as_deref() {
        arguments.extend([
            "-drive".to_string(),
            format!("file={disk_path},format=qcow2"),
        ]);
    }
    if !config.iso_path.trim().is_empty() {
        arguments.extend(["-cdrom".to_string(), config.iso_path.trim().to_string()]);
    }

    Ok(QemuCommandSpec {
        executable_path,
        arguments,
        working_directory: None,
        vm_id: config.id.clone(),
        diagnostics: status.diagnostics.clone(),
        acceleration,
        display_mode,
        network_mode,
    })
}

fn validate_command_inputs(
    config: &VmConfiguration,
    status: &RuntimeStatus,
) -> Result<(), CommandError> {
    config.validate()?;
    if !is_safe_qemu_name(&config.name) {
        return Err(CommandError::validation(
            "name",
            "VM name contains unsupported shell-like characters.",
        ));
    }
    if let Some(path) = config.disk_path.as_deref() {
        validate_path("diskPath", path, true)?;
    }
    if !config.iso_path.trim().is_empty() {
        validate_path("isoPath", &config.iso_path, false)?;
    }
    if status.availability != CapabilityState::Available {
        return Err(CommandError::runtime(
            "QEMU must be detected and version-validated before building a command preview.",
        ));
    }
    validate_executable_path(Path::new(status.detected_qemu_path.as_deref().ok_or_else(
        || CommandError::runtime("No detected QEMU executable is available."),
    )?))
}

fn validate_path(
    field: &'static str,
    value: &str,
    reject_qemu_option_separator: bool,
) -> Result<(), CommandError> {
    let path = value.trim();
    if path.is_empty() {
        return Err(CommandError::validation(field, "Path cannot be empty."));
    }
    if path
        .chars()
        .any(|character| character == '\0' || character.is_control())
    {
        return Err(CommandError::validation(
            field,
            "Path contains invalid control characters.",
        ));
    }
    if reject_qemu_option_separator && path.contains(',') {
        return Err(CommandError::validation(
            field,
            "Disk paths cannot contain commas in the command preview.",
        ));
    }
    #[cfg(windows)]
    if path
        .chars()
        .any(|character| ['<', '>', '"', '|', '?', '*'].contains(&character))
    {
        return Err(CommandError::validation(
            field,
            "Path contains characters that are not valid on Windows.",
        ));
    }
    Ok(())
}

fn validate_executable_path(path: &Path) -> Result<(), CommandError> {
    if path.as_os_str().is_empty() || !path.is_file() {
        return Err(CommandError::validation(
            "executablePath",
            "Executable path must refer to a regular file.",
        ));
    }
    Ok(())
}

fn is_safe_qemu_name(name: &str) -> bool {
    !name.trim().is_empty()
        && !name.chars().any(|character| {
            character.is_control()
                || ['&', '|', ';', '$', '`', '<', '>', '"', '\''].contains(&character)
        })
}

fn select_acceleration(status: &RuntimeStatus) -> QemuAcceleration {
    if status.whpx.state == CapabilityState::Available {
        QemuAcceleration::Whpx
    } else {
        QemuAcceleration::Tcg
    }
}

fn map_display_mode(mode: &DisplayMode) -> Result<QemuDisplayMode, CommandError> {
    match mode {
        DisplayMode::Windowed => Ok(QemuDisplayMode::Sdl),
        DisplayMode::Fullscreen => Ok(QemuDisplayMode::GtkFullscreen),
        DisplayMode::Headless => Ok(QemuDisplayMode::None),
    }
}

fn map_network_mode(mode: &NetworkMode) -> Result<QemuNetworkMode, CommandError> {
    match mode {
        NetworkMode::Disabled => Ok(QemuNetworkMode::None),
        NetworkMode::User => Ok(QemuNetworkMode::User),
        NetworkMode::Bridged => Err(CommandError::unsupported(
            "Bridged networking is not supported by the command preview.",
        )),
    }
}

fn acceleration_argument(acceleration: &QemuAcceleration) -> &'static str {
    match acceleration {
        QemuAcceleration::Whpx => "whpx",
        QemuAcceleration::Tcg => "tcg",
    }
}

fn display_argument(mode: &QemuDisplayMode) -> &'static str {
    match mode {
        QemuDisplayMode::Sdl => "sdl",
        QemuDisplayMode::GtkFullscreen => "gtk,full-screen=on",
        QemuDisplayMode::None => "none",
    }
}

fn network_argument(mode: &QemuNetworkMode) -> &'static str {
    match mode {
        QemuNetworkMode::None => "none",
        QemuNetworkMode::User => "user",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        runtime_adapter::{CapabilityStatus, RuntimeType},
        settings::DisplayMode,
        vm_config::{NetworkMode, OperatingSystem},
    };

    fn config() -> VmConfiguration {
        VmConfiguration {
            id: "vm-test".to_string(),
            name: "Test VM".to_string(),
            operating_system: OperatingSystem::Linux,
            cpu_count: 4,
            memory_mi_b: 8192,
            disk_size_gi_b: 64,
            disk_path: Some("C:\\VMs\\test.qcow2".to_string()),
            iso_path: "C:\\ISOs\\linux.iso".to_string(),
            network_mode: NetworkMode::User,
            display_mode: DisplayMode::Windowed,
            secure_boot_enabled: false,
            tpm_enabled: false,
        }
    }

    fn status() -> RuntimeStatus {
        RuntimeStatus {
            runtime_type: RuntimeType::Qemu,
            availability: CapabilityState::Available,
            configured_qemu_path: None,
            detected_qemu_path: Some("C:\\Program Files\\qemu\\qemu-system-x86_64.exe".to_string()),
            qemu_version: Some("QEMU emulator version 9.0.0".to_string()),
            whpx: CapabilityStatus {
                state: CapabilityState::Unknown,
                detail: String::new(),
            },
            virtualization: CapabilityStatus {
                state: CapabilityState::Unknown,
                detail: String::new(),
            },
            diagnostics: Vec::new(),
            last_checked: String::new(),
        }
    }

    fn test_status() -> RuntimeStatus {
        let mut result = status();
        result.detected_qemu_path = Some(std::env::current_exe().unwrap().display().to_string());
        result
    }

    #[test]
    fn arguments_are_deterministic_and_ordered() {
        let first = build_qemu_command_spec(&config(), &test_status()).unwrap();
        let second = build_qemu_command_spec(&config(), &test_status()).unwrap();
        assert_eq!(first, second);
        assert_eq!(
            &first.arguments[..4],
            &["-name", "Test VM", "-machine", "q35"]
        );
    }

    #[test]
    fn cpu_memory_iso_and_disk_arguments_are_separate() {
        let spec = build_qemu_command_spec(&config(), &test_status()).unwrap();
        assert!(spec.arguments.windows(2).any(|pair| pair == ["-smp", "4"]));
        assert!(spec.arguments.windows(2).any(|pair| pair == ["-m", "8192"]));
        assert!(spec
            .arguments
            .windows(2)
            .any(|pair| pair == ["-cdrom", "C:\\ISOs\\linux.iso"]));
        assert!(spec
            .arguments
            .iter()
            .any(|argument| argument == "file=C:\\VMs\\test.qcow2,format=qcow2"));
    }

    #[test]
    fn display_and_network_modes_map_safely() {
        let spec = build_qemu_command_spec(&config(), &test_status()).unwrap();
        assert_eq!(spec.display_mode, QemuDisplayMode::Sdl);
        assert_eq!(spec.network_mode, QemuNetworkMode::User);
        let mut bridged = config();
        bridged.network_mode = NetworkMode::Bridged;
        assert_eq!(
            build_qemu_command_spec(&bridged, &test_status())
                .unwrap_err()
                .code,
            "unsupported"
        );
    }

    #[test]
    fn whpx_is_selected_only_when_available() {
        let mut runtime_status = test_status();
        runtime_status.whpx.state = CapabilityState::Available;
        let spec = build_qemu_command_spec(&config(), &runtime_status).unwrap();
        assert_eq!(spec.acceleration, QemuAcceleration::Whpx);
    }

    #[test]
    fn invalid_values_and_shell_like_names_are_rejected() {
        let mut invalid = config();
        invalid.name = "unsafe; name".to_string();
        assert_eq!(
            build_qemu_command_spec(&invalid, &test_status())
                .unwrap_err()
                .field,
            Some("name")
        );
        invalid.name = "Test VM".to_string();
        invalid.cpu_count = 0;
        assert_eq!(
            build_qemu_command_spec(&invalid, &test_status())
                .unwrap_err()
                .field,
            Some("cpuCount")
        );
        invalid.cpu_count = 4;
        invalid.disk_path = Some("C:\\bad|path".to_string());
        assert_eq!(
            build_qemu_command_spec(&invalid, &test_status())
                .unwrap_err()
                .field,
            Some("diskPath")
        );
    }

    #[test]
    fn command_spec_serializes_as_executable_and_arguments() {
        let spec = build_qemu_command_spec(&config(), &test_status()).unwrap();
        let serialized = serde_json::to_string(&spec).unwrap();
        assert!(serialized.contains("arguments"));
        assert!(!serialized.contains("cmd.exe"));
    }
}
