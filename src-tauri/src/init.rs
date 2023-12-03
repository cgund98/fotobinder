use std::sync::Arc;

use r2d2_sqlite::SqliteConnectionManager;
use tauri::{Manager, State};

use crate::{biz, data, state::PoolType};

// Type aliases
type InitResult = Result<(), Box<dyn std::error::Error + 'static>>;

// Dependency initializes methods
pub fn init_deps(app: &mut tauri::App) -> InitResult {
    // Initialize tables
    let pool = init_db(app).unwrap();

    // Initialize controllers
    init_controllers(app, pool)?;

    Ok(())
}

fn init_db(app: &mut tauri::App) -> Result<Arc<PoolType>, Box<dyn std::error::Error + 'static>> {
    let handle = app.handle();

    // Read application state
    let binding = handle.path_resolver().app_data_dir().unwrap();

    let data_path = binding.as_path();
    let db_file = data_path.join("main.db");

    // Initialize connection
    let manager = SqliteConnectionManager::file(db_file);
    let pool = r2d2::Pool::new(manager).unwrap();

    crate::data::init::init_db(&pool)?;

    Ok(Arc::new(pool))
}

fn init_controllers(app: &mut tauri::App, pool: Arc<PoolType>) -> InitResult {
    let state: State<crate::state::AppState> = app.state();

    let source_repo = data::source::repo::Repo::new(Arc::clone(&pool));
    let source_controller = biz::source::Controller::new(source_repo);
    *state.source_controller.lock().unwrap() = Some(source_controller);

    let fs_entry_repo = data::fs_entry::repo::Repo::new(Arc::clone(&pool));
    let fs_entry_controller = biz::fs_entry::Controller::new(fs_entry_repo);
    *state.fs_entry_controller.lock().unwrap() = Some(fs_entry_controller);

    Ok(())
}
