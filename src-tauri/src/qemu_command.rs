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
    pub boot_mode: QemuBootMode,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum QemuBootMode {
    Install,
    Normal,
}

pub fn validate_qemu_command_spec(spec: &QemuCommandSpec) -> Result<(), CommandError> {
    if spec.executable_path.as_os_str().is_empty() || !spec.executable_path.is_file() {
        return Err(CommandError::validation(
            "executablePath",
            "Executable path must refer to a regular file.",
        ));
    }
    if spec.arguments.is_empty() {
        return Err(CommandError::validation(
            "arguments",
            "QEMU command arguments cannot be empty.",
        ));
    }
    if spec.vm_id.trim().is_empty() {
        return Err(CommandError::validation("vmId", "VM id is required."));
    }
    if spec.arguments.iter().any(|argument| {
        argument
            .chars()
            .any(|character| character == '\0' || character.is_control())
            || matches!(
                argument.as_str(),
                "cmd.exe" | "powershell" | "pwsh" | "sh" | "-c"
            )
    }) {
        return Err(CommandError::validation(
            "arguments",
            "QEMU arguments contain unsupported process or control input.",
        ));
    }
    Ok(())
}

pub fn validate_iso_path_for_launch(iso_path: &str) -> Result<(), CommandError> {
    let path = iso_path.trim();
    if path.is_empty() {
        return Err(CommandError::validation(
            "isoPath",
            "ISO path must be non-empty when launching a VM that requires an ISO.",
        ));
    }
    let path = Path::new(path);
    if !path.exists() {
        return Err(CommandError::validation(
            "isoPath",
            "ISO path does not exist.",
        ));
    }
    if path.is_dir() {
        return Err(CommandError::validation(
            "isoPath",
            "ISO path must be a regular file, not a directory.",
        ));
    }
    if !path.is_file() {
        return Err(CommandError::validation(
            "isoPath",
            "ISO path must refer to a regular file.",
        ));
    }
    Ok(())
}

pub fn build_qemu_command_spec(
    config: &VmConfiguration,
    status: &RuntimeStatus,
    boot_mode: &QemuBootMode,
) -> Result<QemuCommandSpec, CommandError> {
    validate_command_inputs(config, status, boot_mode)?;

    let executable_path = PathBuf::from(
        status
            .detected_qemu_path
            .as_deref()
            .ok_or_else(|| CommandError::runtime("No detected QEMU executable is available."))?,
    );
    let acceleration = select_acceleration(status);
    let display_mode = map_display_mode(&config.display_mode)?;
    let network_mode = map_network_mode(&config.network_mode)?;
    let boot_argument = match boot_mode {
        QemuBootMode::Install => ("-boot", "order=d"),
        QemuBootMode::Normal => ("-boot", "order=c"),
    };
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
        boot_argument.0.to_string(),
        boot_argument.1.to_string(),
    ];

    if let Some(disk_path) = config.disk_path.as_deref() {
        arguments.extend([
            "-drive".to_string(),
            format!("file={disk_path},format=qcow2"),
        ]);
    }
    if matches!(boot_mode, QemuBootMode::Install) && !config.iso_path.trim().is_empty() {
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
        boot_mode: boot_mode.clone(),
    })
}

fn validate_command_inputs(
    config: &VmConfiguration,
    status: &RuntimeStatus,
    boot_mode: &QemuBootMode,
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
    match boot_mode {
        QemuBootMode::Install => {
            if config.iso_path.trim().is_empty() {
                return Err(CommandError::validation(
                    "bootMode",
                    "Install mode requires an ISO path.",
                ));
            }
        }
        QemuBootMode::Normal => {
            let disk_ready = config
                .disk_path
                .as_deref()
                .map(Path::new)
                .map_or(false, |path| path.is_file());
            if !disk_ready {
                return Err(CommandError::validation(
                    "bootMode",
                    "Normal boot requires an existing persistent disk path.",
                ));
            }
        }
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
            disk_path: Some(
                std::env::current_exe()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            ),
            iso_path: "C:\\ISOs\\linux.iso".to_string(),
            network_mode: NetworkMode::User,
            display_mode: DisplayMode::Windowed,
            secure_boot_enabled: false,
            tpm_enabled: false,
        }
    }

    fn boot_config(boot_mode: QemuBootMode) -> VmConfiguration {
        let mut config = config();
        if matches!(boot_mode, QemuBootMode::Install) {
            config.iso_path = "C:\\ISOs\\linux.iso".to_string();
            config.disk_path = Some(
                std::env::current_exe()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            );
        } else {
            config.iso_path = String::new();
            config.disk_path = Some(
                std::env::current_exe()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            );
        }
        config
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
        let first =
            build_qemu_command_spec(&config(), &test_status(), &QemuBootMode::Normal).unwrap();
        let second =
            build_qemu_command_spec(&config(), &test_status(), &QemuBootMode::Normal).unwrap();
        assert_eq!(first, second);
        assert_eq!(
            &first.arguments[..4],
            &["-name", "Test VM", "-machine", "q35"]
        );
    }

    #[test]
    fn install_mode_attaches_iso_and_disks_normally_boots_disk() {
        let install = build_qemu_command_spec(
            &boot_config(QemuBootMode::Install),
            &test_status(),
            &QemuBootMode::Install,
        )
        .unwrap();
        let normal = build_qemu_command_spec(
            &boot_config(QemuBootMode::Normal),
            &test_status(),
            &QemuBootMode::Normal,
        )
        .unwrap();
        assert!(install
            .arguments
            .windows(2)
            .any(|pair| pair == ["-boot", "order=d"]));
        assert!(install
            .arguments
            .windows(2)
            .any(|pair| pair == ["-cdrom", "C:\\ISOs\\linux.iso"]));
        assert!(install
            .arguments
            .windows(2)
            .any(|pair| pair[0] == "-drive" && pair[1].starts_with("file=")));
        assert!(normal
            .arguments
            .windows(2)
            .any(|pair| pair == ["-boot", "order=c"]));
        assert!(!normal
            .arguments
            .windows(2)
            .any(|pair| pair == ["-cdrom", "C:\\ISOs\\linux.iso"]));
    }

    #[test]
    fn cpu_memory_iso_and_disk_arguments_are_separate() {
        let spec = build_qemu_command_spec(
            &boot_config(QemuBootMode::Install),
            &test_status(),
            &QemuBootMode::Install,
        )
        .unwrap();
        assert!(spec.arguments.windows(2).any(|pair| pair == ["-smp", "4"]));
        assert!(spec.arguments.windows(2).any(|pair| pair == ["-m", "8192"]));
        assert!(spec
            .arguments
            .windows(2)
            .any(|pair| pair == ["-cdrom", "C:\\ISOs\\linux.iso"]));
        assert!(spec
            .arguments
            .iter()
            .any(|argument| argument.starts_with("file=")));
    }

    #[test]
    fn display_and_network_modes_map_safely() {
        let spec = build_qemu_command_spec(
            &boot_config(QemuBootMode::Normal),
            &test_status(),
            &QemuBootMode::Normal,
        )
        .unwrap();
        assert_eq!(spec.display_mode, QemuDisplayMode::Sdl);
        assert_eq!(spec.network_mode, QemuNetworkMode::User);
        let mut bridged = config();
        bridged.network_mode = NetworkMode::Bridged;
        assert_eq!(
            build_qemu_command_spec(&bridged, &test_status(), &QemuBootMode::Normal)
                .unwrap_err()
                .code,
            "unsupported"
        );
    }

    #[test]
    fn whpx_is_selected_only_when_available() {
        let mut runtime_status = test_status();
        runtime_status.whpx.state = CapabilityState::Available;
        let spec = build_qemu_command_spec(
            &boot_config(QemuBootMode::Normal),
            &runtime_status,
            &QemuBootMode::Normal,
        )
        .unwrap();
        assert_eq!(spec.acceleration, QemuAcceleration::Whpx);
    }

    #[test]
    fn invalid_values_and_shell_like_names_are_rejected() {
        let mut invalid = config();
        invalid.name = "unsafe; name".to_string();
        assert_eq!(
            build_qemu_command_spec(&invalid, &test_status(), &QemuBootMode::Normal)
                .unwrap_err()
                .field,
            Some("name")
        );
        invalid.name = "Test VM".to_string();
        invalid.cpu_count = 0;
        assert_eq!(
            build_qemu_command_spec(&invalid, &test_status(), &QemuBootMode::Normal)
                .unwrap_err()
                .field,
            Some("cpuCount")
        );
        invalid.cpu_count = 4;
        invalid.disk_path = Some("C:\\bad|path".to_string());
        assert_eq!(
            build_qemu_command_spec(&invalid, &test_status(), &QemuBootMode::Normal)
                .unwrap_err()
                .field,
            Some("diskPath")
        );
    }

    #[test]
    fn boot_mode_is_serialized_on_command_spec() {
        let spec = build_qemu_command_spec(
            &boot_config(QemuBootMode::Install),
            &test_status(),
            &QemuBootMode::Install,
        )
        .unwrap();
        assert_eq!(spec.boot_mode, QemuBootMode::Install);
        let spec = build_qemu_command_spec(
            &boot_config(QemuBootMode::Normal),
            &test_status(),
            &QemuBootMode::Normal,
        )
        .unwrap();
        assert_eq!(spec.boot_mode, QemuBootMode::Normal);
    }

    #[test]
    fn command_spec_serializes_as_executable_and_arguments() {
        let spec = build_qemu_command_spec(
            &boot_config(QemuBootMode::Normal),
            &test_status(),
            &QemuBootMode::Normal,
        )
        .unwrap();
        let serialized = serde_json::to_string(&spec).unwrap();
        assert!(serialized.contains("arguments"));
        assert!(!serialized.contains("cmd.exe"));
    }

    #[test]
    fn normal_boot_rejects_missing_disk() {
        let mut no_disk = config();
        no_disk.disk_path = None;
        let error =
            build_qemu_command_spec(&no_disk, &test_status(), &QemuBootMode::Normal).unwrap_err();
        assert_eq!(error.field, Some("bootMode"));
        assert!(error.message.contains("disk"));
    }

    #[test]
    fn install_boot_rejects_missing_iso() {
        let mut no_iso = config();
        no_iso.iso_path = String::new();
        let error =
            build_qemu_command_spec(&no_iso, &test_status(), &QemuBootMode::Install).unwrap_err();
        assert_eq!(error.field, Some("bootMode"));
        assert!(error.message.contains("ISO"));
    }

    #[test]
    fn normal_mode_omits_iso_cdrom() {
        let spec =
            build_qemu_command_spec(&config(), &test_status(), &QemuBootMode::Normal).unwrap();
        assert!(!spec.arguments.iter().any(|argument| argument == "-cdrom"));
    }

    #[test]
    fn install_mode_includes_iso_cdrom() {
        let spec = build_qemu_command_spec(
            &boot_config(QemuBootMode::Install),
            &test_status(),
            &QemuBootMode::Install,
        )
        .unwrap();
        assert!(spec.arguments.iter().any(|argument| argument == "-cdrom"));
    }

    #[test]
    fn empty_iso_path_is_rejected() {
        let error = validate_iso_path_for_launch("").unwrap_err();
        assert_eq!(error.field, Some("isoPath"));
        assert!(error.message.contains("non-empty"));
    }

    #[test]
    fn whitespace_only_iso_path_is_rejected() {
        let error = validate_iso_path_for_launch("   ").unwrap_err();
        assert_eq!(error.field, Some("isoPath"));
    }

    #[test]
    fn nonexistent_iso_path_is_rejected() {
        let error = validate_iso_path_for_launch("C:\\ISOs\\does-not-exist.iso").unwrap_err();
        assert_eq!(error.field, Some("isoPath"));
        assert!(error.message.contains("does not exist"));
    }

    #[test]
    fn directory_iso_path_is_rejected() {
        let temp_dir = std::env::temp_dir();
        let error = validate_iso_path_for_launch(temp_dir.to_string_lossy().as_ref()).unwrap_err();
        assert_eq!(error.field, Some("isoPath"));
        assert!(error.message.contains("directory") || error.message.contains("regular file"));
    }

    #[test]
    fn existing_file_iso_path_is_accepted() {
        let exe = std::env::current_exe().unwrap();
        assert!(validate_iso_path_for_launch(exe.to_string_lossy().as_ref()).is_ok());
    }
}
