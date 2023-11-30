use crate::{data::source::entity, state::ServiceAccess};

#[tauri::command]
pub fn create_source(name: &str, handle: tauri::AppHandle) -> Result<entity::DbSource, String> {
    handle
        .source_ctrl(|ctrl| ctrl.create(name))
        .map(entity::DbSource::from)
        .map_err(|error| error.to_string())
}
