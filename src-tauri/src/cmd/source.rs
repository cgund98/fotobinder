use crate::{data::source::entity, errors::AppError, state::ServiceAccess};

#[tauri::command]
pub fn create_source(name: &str, handle: tauri::AppHandle) -> Result<entity::DbSource, AppError> {
    handle
        .source_ctrl(|ctrl| ctrl.create(name))
        .map(entity::DbSource::from)
}

#[tauri::command]
pub fn get_source(id: &str, handle: tauri::AppHandle) -> Result<entity::DbSource, AppError> {
    handle
        .source_ctrl(|ctrl| ctrl.get_by_id(id))
        .map(entity::DbSource::from)
}
