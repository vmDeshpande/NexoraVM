pub mod process_manager;
pub mod qemu_command;
pub mod runtime_adapter;
pub mod settings;
pub mod vm_config;
pub mod vm_definitions;

use process_manager::{
    get_vm_process_status, refresh_all_vm_runtime_status, start_vm, stop_vm, ProcessManagerState,
};
use runtime_adapter::{build_qemu_command_spec, get_runtime_status, refresh_runtime_status};
use settings::{get_app_settings, reset_app_settings, save_app_settings};
use vm_definitions::{
    create_vm_definition, delete_vm_definition, get_vm_definition, list_vm_definitions,
    update_vm_definition,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ProcessManagerState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_app_settings,
            save_app_settings,
            reset_app_settings,
            list_vm_definitions,
            get_vm_definition,
            create_vm_definition,
            update_vm_definition,
            delete_vm_definition,
            start_vm,
            stop_vm,
            get_vm_process_status,
            refresh_all_vm_runtime_status,
            get_runtime_status,
            refresh_runtime_status,
            build_qemu_command_spec
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
