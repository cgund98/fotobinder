use crate::{api::tag::Tags, data::tag::entity, errors::AppError, state::ServiceAccess};

#[tauri::command]
pub fn create_tag(
    name: &str,
    parent_id: Option<&str>,
    handle: tauri::AppHandle,
) -> Result<entity::Tag, AppError> {
    handle.tag_ctrl(|ctrl| ctrl.create(name, parent_id))
}

#[tauri::command]
pub fn get_tag(id: &str, handle: tauri::AppHandle) -> Result<entity::Tag, AppError> {
    handle.tag_ctrl(|ctrl| ctrl.get_by_id(id))
}

#[tauri::command(async)]
pub fn list_tags(handle: tauri::AppHandle) -> Result<Tags, AppError> {
    handle.tag_ctrl(|ctrl| ctrl.list()).map(Tags::from)
}

#[tauri::command(async)]
pub fn list_tags_by_relative_path(
    handle: tauri::AppHandle,
    relative_path: &str,
    source_id: &str,
) -> Result<Tags, AppError> {
    handle
        .tag_ctrl(|ctrl| ctrl.list_by_relative_path(relative_path, source_id))
        .map(Tags::from)
}

#[tauri::command]
pub fn delete_tag(id: &str, handle: tauri::AppHandle) -> Result<(), AppError> {
    handle.tag_ctrl(|ctrl| ctrl.delete(id))
}
