use crate::{
    api::fs_entry::{FsEntries, ScanResults},
    errors::AppError,
    state::ServiceAccess,
};

#[tauri::command(async)]
pub fn scan_source_dir(source_id: &str, handle: tauri::AppHandle) -> Result<ScanResults, AppError> {
    // Get source
    let source = handle.source_ctrl(|ctrl| ctrl.get_by_id(source_id))?;

    handle.fs_entry_ctrl(|ctrl| ctrl.scan_directory(&source.id, &source.path))
}

#[tauri::command(async)]
pub fn generate_missing_thumbnails(
    source_id: &str,
    handle: tauri::AppHandle,
) -> Result<u32, AppError> {
    // Get source
    let source = handle.source_ctrl(|ctrl| ctrl.get_by_id(source_id))?;

    handle.fs_entry_ctrl(|ctrl| ctrl.generate_missing_thumbnails(&source.id, &source.path))
}

#[tauri::command(async)]
pub fn list_fs_entries_by_source_id(
    source_id: &str,
    path_prefix: &str,
    handle: tauri::AppHandle,
) -> Result<FsEntries, AppError> {
    handle
        .fs_entry_ctrl(|ctrl| ctrl.list_by_source_id_and_path_prefix(source_id, path_prefix))
        .map(FsEntries::from)
}

#[tauri::command(async)]
pub fn list_fs_entries_by_collection_id(
    collection_id: &str,
    handle: tauri::AppHandle,
) -> Result<FsEntries, AppError> {
    handle
        .fs_entry_ctrl(|ctrl| ctrl.list_by_collection_id(collection_id))
        .map(FsEntries::from)
}

#[tauri::command(async)]
pub fn list_fs_entries_by_tags(
    includes: Vec<String>,
    excludes: Vec<String>,
    handle: tauri::AppHandle,
) -> Result<FsEntries, AppError> {
    handle
        .fs_entry_ctrl(|ctrl| ctrl.list_by_tags(includes, excludes))
        .map(FsEntries::from)
}

#[tauri::command(async)]
pub fn get_thumbnail_queue_size(handle: tauri::AppHandle) -> Result<usize, AppError> {
    handle.fs_entry_ctrl(|ctrl| ctrl.get_queue_size())
}
