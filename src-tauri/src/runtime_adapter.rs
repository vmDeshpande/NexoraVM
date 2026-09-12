use crate::{
    settings::CommandError,
    vm_config::{NetworkMode, VmConfiguration},
};
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
        if config.id.trim().is_empty() {
            return Err(CommandError::validation("id", "VM id is required."));
        }

        if config.name.trim().is_empty() {
            return Err(CommandError::validation("name", "VM name is required."));
        }

        if config.cpu_count == 0 || config.cpu_count > 128 {
            return Err(CommandError::validation(
                "cpuCount",
                "CPU count must be between 1 and 128.",
            ));
        }

        if !(512..=262_144).contains(&config.memory_mi_b) {
            return Err(CommandError::validation(
                "memoryMiB",
                "Memory must be between 512 MiB and 262144 MiB.",
            ));
        }

        if config.disk_size_gi_b == 0 {
            return Err(CommandError::validation(
                "diskSizeGiB",
                "Disk size must be greater than zero.",
            ));
        }

        if config.iso_path.trim().is_empty() {
            return Err(CommandError::validation("isoPath", "ISO path is required."));
        }

        match config.network_mode {
            NetworkMode::Disabled | NetworkMode::User | NetworkMode::Bridged => {}
        }

        Ok(())
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

fn not_implemented(message: impl Into<String>) -> CommandError {
    CommandError {
        code: "not_implemented",
        message: message.into(),
        field: None,
    }
}
