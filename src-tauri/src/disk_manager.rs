use crate::settings::{AppSettings, CommandError};
use crate::vm_config::VmConfiguration;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const DISK_FORMAT: &str = "qcow2";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiskState {
    NotCreated,
    Ready,
    CreationFailed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskStatus {
    pub vm_id: String,
    pub state: DiskState,
    pub path: Option<String>,
    pub size_gi_b: Option<u32>,
    pub format: Option<String>,
    pub message: Option<String>,
}

pub trait DiskManager: Send + Sync {
    fn create_disk(&self, config: &VmConfiguration) -> Result<DiskStatus, CommandError>;
    fn status(&self, config: &VmConfiguration) -> Result<DiskStatus, CommandError>;
}

#[derive(Default, Clone)]
pub struct QemuDiskManager;

impl DiskManager for QemuDiskManager {
    fn create_disk(&self, config: &VmConfiguration) -> Result<DiskStatus, CommandError> {
        let path = disk_path_or_error(config)?;
        if path.exists() {
            if path.is_dir() {
                return Err(CommandError::validation(
                    "diskPath",
                    "Disk path must be a regular file, not a directory.",
                ));
            }
            return Ok(DiskStatus {
                vm_id: config.id.clone(),
                state: DiskState::Ready,
                path: Some(path.to_string_lossy().into_owned()),
                size_gi_b: Some(config.disk_size_gi_b),
                format: Some(DISK_FORMAT.to_string()),
                message: Some("Disk image already exists; existing image is used.".to_string()),
            });
        }

        let settings = AppSettings::default();
        let qemu_img = find_qemu_img(&settings)?;
        let args = qemu_img_create_args(&path, config.disk_size_gi_b, DISK_FORMAT);
        let output = std::process::Command::new(&qemu_img)
            .args(&args)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
            .map_err(|error| {
                CommandError::runtime(format!("qemu-img could not be started: {error}"))
            })?;

        if output.status.success() {
            return Ok(DiskStatus {
                vm_id: config.id.clone(),
                state: DiskState::Ready,
                path: Some(path.to_string_lossy().into_owned()),
                size_gi_b: Some(config.disk_size_gi_b),
                format: Some(DISK_FORMAT.to_string()),
                message: None,
            });
        }

        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        Ok(DiskStatus {
            vm_id: config.id.clone(),
            state: DiskState::CreationFailed,
            path: Some(path.to_string_lossy().into_owned()),
            size_gi_b: Some(config.disk_size_gi_b),
            format: Some(DISK_FORMAT.to_string()),
            message: Some(stderr),
        })
    }

    fn status(&self, config: &VmConfiguration) -> Result<DiskStatus, CommandError> {
        let path = disk_path_or_error(config)?;
        let ready = path.exists() && path.is_file();
        Ok(DiskStatus {
            vm_id: config.id.clone(),
            state: if ready {
                DiskState::Ready
            } else {
                DiskState::NotCreated
            },
            path: Some(path.to_string_lossy().into_owned()),
            size_gi_b: Some(config.disk_size_gi_b),
            format: Some(DISK_FORMAT.to_string()),
            message: None,
        })
    }
}

fn disk_path_or_error(config: &VmConfiguration) -> Result<PathBuf, CommandError> {
    let path = config.disk_path.as_deref().ok_or_else(|| {
        CommandError::validation("diskPath", "A disk path is required for disk operations.")
    })?;
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        return Err(CommandError::validation(
            "diskPath",
            "Disk path cannot be empty.",
        ));
    }
    Ok(path)
}

fn find_qemu_img(settings: &AppSettings) -> Result<PathBuf, CommandError> {
    if let Some(qemu_path) = settings.qemu_executable_path.as_deref() {
        let qemu_path = PathBuf::from(qemu_path.trim());
        if let Some(parent) = qemu_path.parent() {
            for name in ["qemu-img.exe", "qemu-img"] {
                let candidate = parent.join(name);
                if candidate.is_file() {
                    return Ok(candidate);
                }
            }
        }
    }

    for variable in ["ProgramFiles", "ProgramFiles(x86)"] {
        if let Some(root) = std::env::var_os(variable) {
            let candidate = PathBuf::from(root).join("qemu").join("qemu-img.exe");
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
    }

    let path_variable = std::env::var_os("PATH");
    if let Some(paths) = path_variable {
        for directory in std::env::split_paths(&paths) {
            for name in ["qemu-img.exe", "qemu-img"] {
                let candidate = directory.join(name);
                if candidate.is_file() {
                    return Ok(candidate);
                }
            }
        }
    }

    Err(CommandError::runtime(
        "qemu-img was not detected. Install QEMU or configure the QEMU executable path."
            .to_string(),
    ))
}

pub fn qemu_img_create_args(path: &PathBuf, size_gib: u32, format: &str) -> Vec<String> {
    vec![
        "create".to_string(),
        "-f".to_string(),
        format.to_string(),
        path.to_string_lossy().into_owned(),
        format!("{}G", size_gib),
    ]
}

#[tauri::command]
pub fn create_vm_disk(app: tauri::AppHandle, vm_id: String) -> Result<DiskStatus, CommandError> {
    let definition = crate::vm_definitions::get_vm_definition(app, vm_id)?;
    QemuDiskManager.create_disk(&definition.configuration)
}

#[tauri::command]
pub fn get_vm_disk_status(
    app: tauri::AppHandle,
    vm_id: String,
) -> Result<DiskStatus, CommandError> {
    let definition = crate::vm_definitions::get_vm_definition(app, vm_id)?;
    QemuDiskManager.status(&definition.configuration)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm_config::VmConfiguration;

    fn disk_config(path: &str) -> VmConfiguration {
        VmConfiguration {
            id: "vm-test".to_string(),
            name: "Test VM".to_string(),
            operating_system: crate::vm_config::OperatingSystem::Linux,
            cpu_count: 2,
            memory_mi_b: 4096,
            disk_size_gi_b: 64,
            disk_path: Some(path.to_string()),
            iso_path: String::new(),
            network_mode: crate::vm_config::NetworkMode::User,
            display_mode: crate::settings::DisplayMode::Windowed,
            secure_boot_enabled: false,
            tpm_enabled: false,
        }
    }

    #[test]
    fn qemu_img_create_args_are_deterministic() {
        let path = PathBuf::from("C:\\VMs\\test.qcow2");
        let args = qemu_img_create_args(&path, 64, "qcow2");
        assert_eq!(
            args,
            vec!["create", "-f", "qcow2", "C:\\VMs\\test.qcow2", "64G"]
        );
    }

    #[test]
    fn existing_disk_is_ready_without_overwrite() {
        let temp_file = std::env::temp_dir().join("nexoravm-disk-already-exists.qcow2");
        std::fs::write(&temp_file, b"not a qcow2").unwrap();
        let result = QemuDiskManager
            .create_disk(&disk_config(temp_file.to_string_lossy().as_ref()))
            .unwrap();
        std::fs::remove_file(&temp_file).unwrap();
        assert_eq!(result.state, DiskState::Ready);
        assert_eq!(
            result.message,
            Some("Disk image already exists; existing image is used.".to_string())
        );
    }

    #[test]
    fn directory_disk_path_is_rejected() {
        let temp_dir = std::env::temp_dir().join("nexoravm-disk-dir");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let error = QemuDiskManager
            .create_disk(&disk_config(temp_dir.to_string_lossy().as_ref()))
            .unwrap_err();
        std::fs::remove_dir(&temp_dir).unwrap();
        assert_eq!(error.field, Some("diskPath"));
        assert!(error.message.contains("regular file") || error.message.contains("directory"));
    }

    #[test]
    fn missing_disk_is_not_created() {
        let path = std::env::temp_dir().join("nexoravm-disk-missing.qcow2");
        let status = QemuDiskManager
            .status(&disk_config(path.to_string_lossy().as_ref()))
            .unwrap();
        assert_eq!(status.state, DiskState::NotCreated);
    }

    #[test]
    fn existing_disk_status_reports_ready() {
        let temp_file = std::env::temp_dir().join("nexoravm-disk-status-ready.qcow2");
        std::fs::write(&temp_file, b"qcow").unwrap();
        let status = QemuDiskManager
            .status(&disk_config(temp_file.to_string_lossy().as_ref()))
            .unwrap();
        std::fs::remove_file(&temp_file).unwrap();
        assert_eq!(status.state, DiskState::Ready);
    }
}
