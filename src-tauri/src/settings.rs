use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};
use tauri::{AppHandle, Manager};

const SETTINGS_FILE_NAME: &str = "settings.json";
const MIN_MEMORY_MIB: u32 = 512;
const MAX_MEMORY_MIB: u32 = 262_144;
const MIN_CPU_COUNT: u8 = 1;
const MAX_CPU_COUNT: u8 = 128;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandError {
    pub code: &'static str,
    pub message: String,
    pub field: Option<&'static str>,
}

impl CommandError {
    pub fn validation(field: &'static str, message: impl Into<String>) -> Self {
        Self {
            code: "validation_error",
            message: message.into(),
            field: Some(field),
        }
    }

    pub fn storage(message: impl Into<String>) -> Self {
        Self {
            code: "storage_error",
            message: message.into(),
            field: None,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            code: "not_found",
            message: message.into(),
            field: None,
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            code: "conflict",
            message: message.into(),
            field: None,
        }
    }
}

impl From<io::Error> for CommandError {
    fn from(error: io::Error) -> Self {
        Self::storage(error.to_string())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub qemu_executable_path: Option<String>,
    pub default_vm_storage_path: Option<String>,
    pub default_iso_path: Option<String>,
    pub default_memory_mi_b: u32,
    pub default_cpu_count: u8,
    pub preferred_display_mode: DisplayMode,
    pub start_minimized: bool,
    pub check_for_updates: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DisplayMode {
    Windowed,
    Fullscreen,
    Headless,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            qemu_executable_path: None,
            default_vm_storage_path: None,
            default_iso_path: None,
            default_memory_mi_b: 4096,
            default_cpu_count: 2,
            preferred_display_mode: DisplayMode::Windowed,
            start_minimized: false,
            check_for_updates: true,
        }
    }
}

impl AppSettings {
    pub fn validate(&self) -> Result<(), CommandError> {
        validate_optional_path("qemuExecutablePath", self.qemu_executable_path.as_deref())?;
        validate_optional_path(
            "defaultVmStoragePath",
            self.default_vm_storage_path.as_deref(),
        )?;
        validate_optional_path("defaultIsoPath", self.default_iso_path.as_deref())?;

        if !(MIN_MEMORY_MIB..=MAX_MEMORY_MIB).contains(&self.default_memory_mi_b) {
            return Err(CommandError::validation(
                "defaultMemoryMiB",
                format!("Memory must be between {MIN_MEMORY_MIB} MiB and {MAX_MEMORY_MIB} MiB."),
            ));
        }

        if !(MIN_CPU_COUNT..=MAX_CPU_COUNT).contains(&self.default_cpu_count) {
            return Err(CommandError::validation(
                "defaultCpuCount",
                format!("CPU count must be between {MIN_CPU_COUNT} and {MAX_CPU_COUNT}."),
            ));
        }

        Ok(())
    }
}

#[tauri::command]
pub fn get_app_settings(app: AppHandle) -> Result<AppSettings, CommandError> {
    let settings_path = settings_file_path(&app)?;

    if !settings_path.exists() {
        return Ok(AppSettings::default());
    }

    let content = fs::read_to_string(settings_path)?;
    Ok(parse_settings_or_default(&content))
}

#[tauri::command]
pub fn save_app_settings(
    app: AppHandle,
    settings: AppSettings,
) -> Result<AppSettings, CommandError> {
    settings.validate()?;

    let settings_path = settings_file_path(&app)?;
    if let Some(settings_dir) = settings_path.parent() {
        fs::create_dir_all(settings_dir)?;
    }

    let content = serde_json::to_string_pretty(&settings)
        .map_err(|error| CommandError::storage(error.to_string()))?;
    fs::write(settings_path, content)?;

    Ok(settings)
}

#[tauri::command]
pub fn reset_app_settings(app: AppHandle) -> Result<AppSettings, CommandError> {
    let settings = AppSettings::default();
    save_app_settings(app, settings)
}

fn settings_file_path(app: &AppHandle) -> Result<PathBuf, CommandError> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| CommandError::storage(error.to_string()))?;

    Ok(app_data_dir.join(SETTINGS_FILE_NAME))
}

fn validate_optional_path(field: &'static str, value: Option<&str>) -> Result<(), CommandError> {
    let Some(path) = value else {
        return Ok(());
    };

    let path = path.trim();
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

    #[cfg(windows)]
    {
        let invalid_characters = ['<', '>', '"', '|', '?', '*'];
        if path
            .chars()
            .any(|character| invalid_characters.contains(&character))
        {
            return Err(CommandError::validation(
                field,
                "Path contains characters that are not valid on Windows.",
            ));
        }
    }

    Ok(())
}

fn parse_settings_or_default(content: &str) -> AppSettings {
    let Ok(settings) = serde_json::from_str::<AppSettings>(content) else {
        return AppSettings::default();
    };

    if settings.validate().is_err() {
        return AppSettings::default();
    }

    settings
}

#[cfg(test)]
mod tests {
    use super::{parse_settings_or_default, AppSettings};

    #[test]
    fn malformed_settings_recover_to_defaults() {
        assert_eq!(
            parse_settings_or_default("{ not valid json"),
            AppSettings::default()
        );
    }

    #[test]
    fn invalid_settings_recover_to_defaults() {
        let invalid_settings = r#"{
            "qemuExecutablePath": null,
            "defaultVmStoragePath": null,
            "defaultIsoPath": null,
            "defaultMemoryMiB": 128,
            "defaultCpuCount": 2,
            "preferredDisplayMode": "windowed",
            "startMinimized": false,
            "checkForUpdates": true
        }"#;

        assert_eq!(
            parse_settings_or_default(invalid_settings),
            AppSettings::default()
        );
    }
}
