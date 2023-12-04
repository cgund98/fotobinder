use crate::{api::fs_entry::FsEntries, errors::AppError, state::ServiceAccess};

#[tauri::command]
pub fn scan_source_dir(source_id: &str, handle: tauri::AppHandle) -> Result<(), AppError> {
    // Parse thumbnail directory
    let binding = handle.path_resolver().app_data_dir().unwrap();
    let thumbnail_buf = binding.join("thumbnails");
    let thumbnail_path = thumbnail_buf.as_path();
    println!("Creating path...: {}", thumbnail_path.to_string_lossy());
    std::fs::create_dir_all(thumbnail_path)?;

    let source = handle.source_ctrl(|ctrl| ctrl.get_by_id(source_id))?;

    handle.fs_entry_ctrl(|ctrl| ctrl.scan_directory(source.id, source.path, thumbnail_path))
}

#[tauri::command]
pub fn list_fs_entries_by_source_id(
    source_id: &str,
    path_prefix: &str,
    handle: tauri::AppHandle,
) -> Result<FsEntries, AppError> {
    handle
        .fs_entry_ctrl(|ctrl| ctrl.list_by_source_id_and_path_prefix(source_id, path_prefix))
        .map(FsEntries::from)
}
