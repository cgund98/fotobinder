use crate::{
    api::collection::Collections, data::collection::entity, errors::AppError, state::ServiceAccess,
};

#[tauri::command]
pub fn create_collection(
    name: &str,
    parent_id: Option<&str>,
    handle: tauri::AppHandle,
) -> Result<entity::Collection, AppError> {
    handle.collection_ctrl(|ctrl| ctrl.create(name, parent_id))
}

#[tauri::command]
pub fn get_collection(id: &str, handle: tauri::AppHandle) -> Result<entity::Collection, AppError> {
    handle.collection_ctrl(|ctrl| ctrl.get_by_id(id))
}

#[tauri::command]
pub fn update_collection(
    id: &str,
    name: &str,
    parent_id: Option<&str>,
    handle: tauri::AppHandle,
) -> Result<entity::Collection, AppError> {
    handle.collection_ctrl(|ctrl| ctrl.update_by_id(id, name, parent_id))
}

#[tauri::command(async)]
pub fn list_collections(handle: tauri::AppHandle) -> Result<Collections, AppError> {
    handle
        .collection_ctrl(|ctrl| ctrl.list())
        .map(Collections::from)
}

#[tauri::command(async)]
pub fn list_collections_by_parent_id(
    handle: tauri::AppHandle,
    parent_id: Option<String>,
) -> Result<Collections, AppError> {
    handle
        .collection_ctrl(|ctrl| ctrl.list_by_parent_id(parent_id))
        .map(Collections::from)
}

#[tauri::command]
pub fn delete_collection(id: &str, handle: tauri::AppHandle) -> Result<(), AppError> {
    handle.collection_ctrl(|ctrl| ctrl.delete(id))
}
