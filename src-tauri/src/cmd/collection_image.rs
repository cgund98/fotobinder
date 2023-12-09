use crate::{
    api::collection_image::CollectionImages, data::collection_image::entity, errors::AppError,
    state::ServiceAccess,
};

#[tauri::command]
pub fn create_collection_image(
    collection_id: &str,
    relative_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<entity::CollectionImage, AppError> {
    handle.collection_image_ctrl(|ctrl| ctrl.create(collection_id, relative_path, source_id))
}

#[tauri::command]
pub fn get_collection_image(
    collection_id: &str,
    relative_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<entity::CollectionImage, AppError> {
    handle.collection_image_ctrl(|ctrl| ctrl.get_by_ids(collection_id, relative_path, source_id))
}

#[tauri::command(async)]
pub fn list_collection_images(handle: tauri::AppHandle) -> Result<CollectionImages, AppError> {
    handle
        .collection_image_ctrl(|ctrl| ctrl.list())
        .map(CollectionImages::from)
}

#[tauri::command(async)]
pub fn list_collection_images_by_relative_path(
    relative_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<CollectionImages, AppError> {
    handle
        .collection_image_ctrl(|ctrl| ctrl.list_by_relative_path(relative_path, source_id))
        .map(CollectionImages::from)
}

#[tauri::command]
pub fn delete_collection_image(
    collection_id: &str,
    relative_path: &str,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<(), AppError> {
    handle.collection_image_ctrl(|ctrl| ctrl.delete(collection_id, relative_path, source_id))
}

#[tauri::command]
pub fn delete_many_collection_images(
    collection_id: &str,
    relative_paths: Vec<String>,
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<(), AppError> {
    handle.collection_image_ctrl(|ctrl| ctrl.delete_many(collection_id, relative_paths, source_id))
}

#[tauri::command]
pub fn assign_many_collection_images(
    collection_id: &str,
    relative_paths: Vec<String>,
    source_ids: Vec<String>,
    handle: tauri::AppHandle,
) -> Result<(), AppError> {
    handle.collection_image_ctrl(|ctrl| ctrl.assign_many(collection_id, relative_paths, source_ids))
}
