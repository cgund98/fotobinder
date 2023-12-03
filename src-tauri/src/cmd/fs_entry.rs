use crate::{errors::AppError, state::ServiceAccess};

#[tauri::command]
pub fn scan_source_dir(source_id: &str, handle: tauri::AppHandle) -> Result<(), AppError> {
    let source = handle.source_ctrl(|ctrl| ctrl.get_by_id(source_id))?;

    handle.fs_entry_ctrl(|ctrl| ctrl.scan_directory(source.id, source.path))
}
