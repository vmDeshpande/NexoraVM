use crate::{settings::CommandError, vm_config::VmConfiguration};
use serde::{Deserialize, Serialize};

pub trait RuntimeAdapter {
    fn check_availability(&self) -> Result<RuntimeAvailability, CommandError>;
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
pub fn start_vm(vm_id: String) -> Result<(), CommandError> {
    QemuRuntimeAdapter.start_vm(&vm_id)
}

#[tauri::command]
pub fn stop_vm(vm_id: String) -> Result<(), CommandError> {
    QemuRuntimeAdapter.stop_vm(&vm_id)
}

fn not_implemented(message: impl Into<String>) -> CommandError {
    CommandError {
        code: "not_implemented",
        message: message.into(),
        field: None,
    }
}
