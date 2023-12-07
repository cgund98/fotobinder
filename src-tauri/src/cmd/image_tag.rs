use crate::{
    api::image_tag::{ImageTags, TagAssignments},
    data::image_tag::entity,
    errors::AppError,
    state::ServiceAccess,
};

#[tauri::command]
pub fn create_image_tag(
    tag_id: &str,
    relative_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<entity::ImageTag, AppError> {
    handle.image_tag_ctrl(|ctrl| ctrl.create(tag_id, relative_path, source_id))
}

#[tauri::command]
pub fn get_image_tag(
    tag_id: &str,
    relative_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<entity::ImageTag, AppError> {
    handle.image_tag_ctrl(|ctrl| ctrl.get_by_ids(tag_id, relative_path, source_id))
}

#[tauri::command(async)]
pub fn list_image_tags(handle: tauri::AppHandle) -> Result<ImageTags, AppError> {
    handle
        .image_tag_ctrl(|ctrl| ctrl.list())
        .map(ImageTags::from)
}

#[tauri::command(async)]
pub fn list_image_tags_by_relative_path(
    relative_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<ImageTags, AppError> {
    handle
        .image_tag_ctrl(|ctrl| ctrl.list_by_relative_path(relative_path, source_id))
        .map(ImageTags::from)
}

#[tauri::command]
pub fn delete_image_tag(
    tag_id: &str,
    relative_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<(), AppError> {
    handle.image_tag_ctrl(|ctrl| ctrl.delete(tag_id, relative_path, source_id))
}

#[tauri::command]
pub fn assign_image_tags(
    relative_paths: Vec<String>,
    source_id: &str,
    assignments: TagAssignments,
    handle: tauri::AppHandle,
) -> Result<(), AppError> {
    handle.image_tag_ctrl(|ctrl| ctrl.assign(relative_paths, source_id, assignments))
}
