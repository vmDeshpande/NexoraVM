use crate::settings::DisplayMode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VmConfiguration {
    pub id: String,
    pub name: String,
    pub operating_system: OperatingSystem,
    pub cpu_count: u8,
    pub memory_mi_b: u32,
    pub disk_size_gi_b: u32,
    pub iso_path: String,
    pub network_mode: NetworkMode,
    pub display_mode: DisplayMode,
    pub secure_boot_enabled: bool,
    pub tpm_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OperatingSystem {
    Windows,
    Linux,
    Bsd,
    Other,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NetworkMode {
    Disabled,
    User,
    Bridged,
}
