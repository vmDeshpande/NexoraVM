use crate::settings::{CommandError, DisplayMode};
use serde::{Deserialize, Serialize};

const MAX_NAME_LENGTH: usize = 100;
const MIN_MEMORY_MIB: u32 = 512;
const MAX_MEMORY_MIB: u32 = 262_144;
const MAX_CPU_COUNT: u8 = 128;
const MAX_DISK_SIZE_GIB: u32 = 1_048_576;
const MAX_ID_LENGTH: usize = 64;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VmConfiguration {
    pub id: String,
    pub name: String,
    pub operating_system: OperatingSystem,
    pub cpu_count: u8,
    pub memory_mi_b: u32,
    pub disk_size_gi_b: u32,
    #[serde(default)]
    pub disk_path: Option<String>,
    pub iso_path: String,
    pub network_mode: NetworkMode,
    pub display_mode: DisplayMode,
    pub secure_boot_enabled: bool,
    pub tpm_enabled: bool,
}

impl VmConfiguration {
    pub fn validate(&self) -> Result<(), CommandError> {
        validate_vm_id(&self.id)?;

        let name = self.name.trim();
        if name.is_empty() {
            return Err(CommandError::validation("name", "VM name is required."));
        }
        if name.chars().count() > MAX_NAME_LENGTH {
            return Err(CommandError::validation(
                "name",
                format!("VM name cannot exceed {MAX_NAME_LENGTH} characters."),
            ));
        }

        if self.cpu_count == 0 || self.cpu_count > MAX_CPU_COUNT {
            return Err(CommandError::validation(
                "cpuCount",
                format!("CPU count must be between 1 and {MAX_CPU_COUNT}."),
            ));
        }

        if !(MIN_MEMORY_MIB..=MAX_MEMORY_MIB).contains(&self.memory_mi_b) {
            return Err(CommandError::validation(
                "memoryMiB",
                format!("Memory must be between {MIN_MEMORY_MIB} MiB and {MAX_MEMORY_MIB} MiB."),
            ));
        }

        if self.disk_size_gi_b == 0 || self.disk_size_gi_b > MAX_DISK_SIZE_GIB {
            return Err(CommandError::validation(
                "diskSizeGiB",
                format!("Disk size must be between 1 GiB and {MAX_DISK_SIZE_GIB} GiB."),
            ));
        }

        validate_optional_path("isoPath", Some(&self.iso_path))
    }
}

fn validate_vm_id(id: &str) -> Result<(), CommandError> {
    let id = id.trim();
    if id.is_empty() {
        return Err(CommandError::validation("id", "VM id is required."));
    }
    if id.chars().count() > MAX_ID_LENGTH {
        return Err(CommandError::validation(
            "id",
            format!("VM id cannot exceed {MAX_ID_LENGTH} characters."),
        ));
    }
    if !id
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '-' || character == '_')
    {
        return Err(CommandError::validation(
            "id",
            "VM id may contain only letters, numbers, hyphens, and underscores.",
        ));
    }
    Ok(())
}

fn validate_optional_path(field: &'static str, value: Option<&str>) -> Result<(), CommandError> {
    let Some(path) = value else {
        return Ok(());
    };

    let path = path.trim();
    if path.is_empty() {
        return Ok(());
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OperatingSystem {
    Windows,
    Linux,
    Bsd,
    Other,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NetworkMode {
    Disabled,
    User,
    Bridged,
}

#[cfg(test)]
mod tests {
    use super::{NetworkMode, OperatingSystem, VmConfiguration};
    use crate::settings::DisplayMode;

    fn valid_configuration() -> VmConfiguration {
        VmConfiguration {
            id: "vm-test".to_string(),
            name: "Test VM".to_string(),
            operating_system: OperatingSystem::Linux,
            cpu_count: 2,
            memory_mi_b: 4096,
            disk_size_gi_b: 64,
            disk_path: None,
            iso_path: String::new(),
            network_mode: NetworkMode::User,
            display_mode: DisplayMode::Windowed,
            secure_boot_enabled: false,
            tpm_enabled: false,
        }
    }

    #[test]
    fn valid_configuration_passes_validation() {
        assert!(valid_configuration().validate().is_ok());
    }

    #[test]
    fn invalid_resources_and_name_are_rejected() {
        let mut configuration = valid_configuration();
        configuration.cpu_count = 0;
        assert_eq!(
            configuration.validate().unwrap_err().field,
            Some("cpuCount")
        );

        configuration.cpu_count = 2;
        configuration.memory_mi_b = 128;
        assert_eq!(
            configuration.validate().unwrap_err().field,
            Some("memoryMiB")
        );

        configuration.memory_mi_b = 4096;
        configuration.disk_size_gi_b = 0;
        assert_eq!(
            configuration.validate().unwrap_err().field,
            Some("diskSizeGiB")
        );

        configuration.disk_size_gi_b = 64;
        configuration.name = "  ".to_string();
        assert_eq!(configuration.validate().unwrap_err().field, Some("name"));
    }

    #[test]
    fn configuration_serializes_with_frontend_field_names() {
        let serialized = serde_json::to_string(&valid_configuration()).unwrap();
        assert!(serialized.contains("\"memoryMiB\""));
        assert!(serialized.contains("\"diskSizeGiB\""));

        let restored: VmConfiguration = serde_json::from_str(&serialized).unwrap();
        assert_eq!(restored.name, "Test VM");
    }
}
