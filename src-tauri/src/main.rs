// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fotobinder::cmd;

fn main() {
    tauri::Builder::default()
        .manage(fotobinder::state::AppState::default())
        .setup(fotobinder::init::init_deps)
        .invoke_handler(tauri::generate_handler![
            cmd::source::create_source,
            cmd::source::get_source,
            cmd::source::delete_source,
            cmd::source::list_sources,
            cmd::fs_entry::scan_source_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
