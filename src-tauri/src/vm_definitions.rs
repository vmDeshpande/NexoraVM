use crate::{
    settings::{CommandError, DisplayMode},
    vm_config::{NetworkMode, OperatingSystem, VmConfiguration},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::PathBuf};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

const VM_DEFINITIONS_FILE_NAME: &str = "vm-definitions.json";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VmDefinition {
    pub configuration: VmConfiguration,
}

#[tauri::command]
pub fn list_vm_definitions(app: AppHandle) -> Result<Vec<VmDefinition>, CommandError> {
    load_definitions(&app)
}

#[tauri::command]
pub fn get_vm_definition(app: AppHandle, vm_id: String) -> Result<VmDefinition, CommandError> {
    validate_id(&vm_id)?;
    load_definitions(&app)?
        .into_iter()
        .find(|definition| definition.configuration.id == vm_id)
        .ok_or_else(|| CommandError::not_found("The requested VM definition was not found."))
}

#[tauri::command]
pub fn create_vm_definition(
    app: AppHandle,
    mut configuration: VmConfiguration,
) -> Result<VmDefinition, CommandError> {
    configuration.id = format!("vm-{}", Uuid::new_v4().simple());
    configuration.validate()?;

    let mut definitions = load_definitions(&app)?;
    let definition = VmDefinition { configuration };
    if definitions
        .iter()
        .any(|item| item.configuration.id == definition.configuration.id)
    {
        return Err(CommandError::conflict(
            "A VM definition with this generated id already exists.",
        ));
    }
    definitions.push(definition.clone());
    save_definitions(&app, &definitions)?;
    Ok(definition)
}

#[tauri::command]
pub fn update_vm_definition(
    app: AppHandle,
    configuration: VmConfiguration,
) -> Result<VmDefinition, CommandError> {
    configuration.validate()?;
    let mut definitions = load_definitions(&app)?;
    let definition = definitions
        .iter_mut()
        .find(|item| item.configuration.id == configuration.id)
        .ok_or_else(|| CommandError::not_found("The requested VM definition was not found."))?;
    definition.configuration = configuration;
    let updated_definition = definition.clone();
    save_definitions(&app, &definitions)?;
    Ok(updated_definition)
}

#[tauri::command]
pub fn delete_vm_definition(app: AppHandle, vm_id: String) -> Result<(), CommandError> {
    validate_id(&vm_id)?;
    let mut definitions = load_definitions(&app)?;
    let original_length = definitions.len();
    definitions.retain(|item| item.configuration.id != vm_id);
    if definitions.len() == original_length {
        return Err(CommandError::not_found(
            "The requested VM definition was not found.",
        ));
    }
    save_definitions(&app, &definitions)
}

fn load_definitions(app: &AppHandle) -> Result<Vec<VmDefinition>, CommandError> {
    let path = definitions_file_path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    let definitions = parse_definitions_or_empty(&content);
    validate_loaded_definitions(&definitions)?;
    Ok(definitions)
}

fn parse_definitions_or_empty(content: &str) -> Vec<VmDefinition> {
    serde_json::from_str::<Vec<VmDefinition>>(content).unwrap_or_default()
}

fn save_definitions(app: &AppHandle, definitions: &[VmDefinition]) -> Result<(), CommandError> {
    let path = definitions_file_path(app)?;
    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory)?;
    }
    let content = serde_json::to_string_pretty(definitions)
        .map_err(|error| CommandError::storage(error.to_string()))?;
    fs::write(path, content)?;
    Ok(())
}

fn definitions_file_path(app: &AppHandle) -> Result<PathBuf, CommandError> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|error| CommandError::storage(error.to_string()))?
        .join(VM_DEFINITIONS_FILE_NAME))
}

fn validate_loaded_definitions(definitions: &[VmDefinition]) -> Result<(), CommandError> {
    let mut ids = HashSet::new();
    for definition in definitions {
        definition.configuration.validate()?;
        if !ids.insert(definition.configuration.id.clone()) {
            return Err(CommandError::conflict(
                "VM storage contains duplicate definition ids.",
            ));
        }
    }
    Ok(())
}

fn validate_id(id: &str) -> Result<(), CommandError> {
    let configuration = VmConfiguration {
        id: id.to_string(),
        name: "placeholder".to_string(),
        operating_system: OperatingSystem::Other,
        cpu_count: 1,
        memory_mi_b: 512,
        disk_size_gi_b: 1,
        disk_path: None,
        iso_path: String::new(),
        network_mode: NetworkMode::Disabled,
        display_mode: DisplayMode::Headless,
        secure_boot_enabled: false,
        tpm_enabled: false,
        boot_mode: crate::qemu_command::QemuBootMode::Normal,
    };
    configuration.validate().map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::{parse_definitions_or_empty, validate_loaded_definitions, VmDefinition};

    #[test]
    fn definitions_serialize_configuration_without_runtime_status() {
        let definition: VmDefinition = serde_json::from_str(
            r#"{
                    "status": "running",
                    "configuration": {
                    "id": "vm-test",
                    "name": "Test VM",
                    "operatingSystem": "linux",
                    "cpuCount": 2,
                    "memoryMiB": 4096,
                    "diskSizeGiB": 64,
                    "isoPath": "",
                    "networkMode": "user",
                    "displayMode": "windowed",
                    "secureBootEnabled": false,
                    "tpmEnabled": false
                    }
                }"#,
        )
        .unwrap();
        let serialized = serde_json::to_string(&definition).unwrap();
        assert!(!serialized.contains("status"));
    }

    #[test]
    fn malformed_storage_recovers_to_empty_definitions() {
        assert!(parse_definitions_or_empty("not valid json").is_empty());
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let definition: VmDefinition = serde_json::from_str(
            r#"{
                "configuration": {
                    "id": "vm-duplicate",
                    "name": "Test VM",
                    "operatingSystem": "linux",
                    "cpuCount": 2,
                    "memoryMiB": 4096,
                    "diskSizeGiB": 64,
                    "isoPath": "",
                    "networkMode": "user",
                    "displayMode": "windowed",
                    "secureBootEnabled": false,
                    "tpmEnabled": false
                },
                "status": "stopped"
            }"#,
        )
        .unwrap();

        let error = validate_loaded_definitions(&[definition.clone(), definition]).unwrap_err();
        assert_eq!(error.code, "conflict");
    }

    #[test]
    fn missing_ids_are_rejected() {
        let definitions = parse_definitions_or_empty(
            r#"[{
                "configuration": {
                    "id": "",
                    "name": "Test VM",
                    "operatingSystem": "linux",
                    "cpuCount": 2,
                    "memoryMiB": 4096,
                    "diskSizeGiB": 64,
                    "isoPath": "",
                    "networkMode": "user",
                    "displayMode": "windowed",
                    "secureBootEnabled": false,
                    "tpmEnabled": false
                },
                "status": "stopped"
            }]"#,
        );
        assert_eq!(
            validate_loaded_definitions(&definitions).unwrap_err().field,
            Some("id")
        );
    }
}
