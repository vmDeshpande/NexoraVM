pub mod runtime_adapter;
pub mod settings;
pub mod vm_config;

use settings::{get_app_settings, reset_app_settings, save_app_settings};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_app_settings,
            save_app_settings,
            reset_app_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
