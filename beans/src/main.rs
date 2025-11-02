// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::AppState;
use std::sync::Mutex;

fn main() {
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            ledger: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            commands::open_ledger,
            commands::create_ledger,
            commands::add_entry,
            commands::update_entry,
            commands::delete_entry,
            commands::get_entries,
            commands::get_entries_filtered,
            commands::get_report_data,
            commands::export_ledger,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
