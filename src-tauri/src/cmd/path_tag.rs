use crate::{
    api::path_tag::PathTags, data::path_tag::entity, errors::AppError, state::ServiceAccess,
};

#[tauri::command]
pub fn create_path_tag(
    tag_id: &str,
    base_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<entity::PathTag, AppError> {
    handle.path_tag_ctrl(|ctrl| ctrl.create(tag_id, base_path, source_id))
}

#[tauri::command]
pub fn get_path_tag(
    tag_id: &str,
    base_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<entity::PathTag, AppError> {
    handle.path_tag_ctrl(|ctrl| ctrl.get_by_ids(tag_id, base_path, source_id))
}

#[tauri::command(async)]
pub fn list_path_tags(handle: tauri::AppHandle) -> Result<PathTags, AppError> {
    handle.path_tag_ctrl(|ctrl| ctrl.list()).map(PathTags::from)
}

#[tauri::command(async)]
pub fn list_path_tags_by_base_path(
    base_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<PathTags, AppError> {
    handle
        .path_tag_ctrl(|ctrl| ctrl.list_by_base_path(base_path, source_id))
        .map(PathTags::from)
}

#[tauri::command]
pub fn delete_path_tag(
    tag_id: &str,
    base_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<(), AppError> {
    handle.path_tag_ctrl(|ctrl| ctrl.delete(tag_id, base_path, source_id))
}
