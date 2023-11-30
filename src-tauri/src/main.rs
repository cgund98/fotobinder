// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use fotobinder::cmd;

fn main() {
    tauri::Builder::default()
        .manage(fotobinder::state::AppState::default())
        .setup(fotobinder::init::init_deps)
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![cmd::source::create_source])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
