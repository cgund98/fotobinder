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

fn init_db(app: &mut tauri::App) -> Result<PoolType, Box<dyn std::error::Error + 'static>> {
    let handle = app.handle();

    // Read application state
    let binding = handle.path_resolver().app_data_dir().unwrap();

    let data_path = binding.as_path();
    let db_file = data_path.join("main.db");

    // Initialize connection
    let manager = SqliteConnectionManager::file(db_file);
    let pool = r2d2::Pool::new(manager).unwrap();

    crate::data::init::init_db(&pool)?;

    Ok(pool)
}

fn init_controllers(app: &mut tauri::App, pool: PoolType) -> InitResult {
    let state: State<crate::state::AppState> = app.state();

    let source_repo = data::source::repo::new(pool);
    let source_controller = biz::source::new_controller(source_repo);
    *state.source_controller.lock().unwrap() = Some(source_controller);

    Ok(())
}
