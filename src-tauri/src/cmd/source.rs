use crate::{
    api::source::{Source, Sources},
    errors::AppError,
    state::ServiceAccess,
};

#[tauri::command]
pub fn validate_source_payload(
    name: &str,
    source_type: &str,
    path: &str,
    handle: tauri::AppHandle,
) -> Result<(), AppError> {
    handle.source_ctrl(|ctrl| ctrl.validate_payload(name, source_type, path))
}

#[tauri::command]
pub fn create_source(
    name: &str,
    source_type: &str,
    path: &str,
    handle: tauri::AppHandle,
) -> Result<Source, AppError> {
    handle
        .source_ctrl(|ctrl| ctrl.create(name, source_type, path))
        .map(Source::from)
}

#[tauri::command]
pub fn get_source(id: &str, handle: tauri::AppHandle) -> Result<Source, AppError> {
    handle
        .source_ctrl(|ctrl| ctrl.get_by_id(id))
        .map(Source::from)
}

#[tauri::command(async)]
pub fn list_sources(handle: tauri::AppHandle) -> Result<Sources, AppError> {
    handle.source_ctrl(|ctrl| ctrl.list()).map(Sources::from)
}

#[tauri::command]
pub fn delete_source(id: &str, handle: tauri::AppHandle) -> Result<(), AppError> {
    handle.source_ctrl(|ctrl| ctrl.delete(id))
}
