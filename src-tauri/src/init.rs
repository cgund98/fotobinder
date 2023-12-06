use std::sync::Arc;

use r2d2_sqlite::SqliteConnectionManager;
use tauri::{Manager, State};

use crate::{biz, data, fs::queue, state::PoolType};

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

    // Find app path
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
    // Initialize thumbnail queue
    let th_queue = queue::init_queue();

    // Read app state
    let state: State<crate::state::AppState> = app.state();

    // Find thumbnails path
    let binding = app.app_handle().path_resolver().app_data_dir().unwrap();
    let thumbnail_buf = binding.join("thumbnails");
    let thumbnails_path = thumbnail_buf.as_path();
    let thumbnails_path_str = String::from(thumbnails_path.to_string_lossy());
    std::fs::create_dir_all(thumbnails_path)?;

    // Init source controller
    let source_repo = data::source::repo::Repo::new(Arc::clone(&pool));
    let source_controller = biz::source::Controller::new(source_repo);
    *state.source_controller.lock().unwrap() = Some(source_controller);

    // Init fs_entry controller
    let ctrl_th_queue = th_queue.clone();
    let fs_entry_repo = Arc::new(data::fs_entry::repo::Repo::new(Arc::clone(&pool)));
    let fs_entry_controller =
        biz::fs_entry::Controller::new(fs_entry_repo.clone(), ctrl_th_queue, thumbnails_path_str);
    *state.fs_entry_controller.lock().unwrap() = Some(fs_entry_controller);

    // Init tags controller
    let tag_repo = data::tag::repo::Repo::new(Arc::clone(&pool));
    let tag_controller = biz::tag::Controller::new(tag_repo);
    *state.tag_controller.lock().unwrap() = Some(tag_controller);

    // Init path tags controller
    let path_tag_repo = data::path_tag::repo::Repo::new(Arc::clone(&pool));
    let path_tag_controller = biz::path_tag::Controller::new(path_tag_repo, fs_entry_repo.clone());
    *state.path_tag_controller.lock().unwrap() = Some(path_tag_controller);

    // Init image tags controller
    let img_tag_repo = data::image_tag::repo::Repo::new(Arc::clone(&pool));
    let img_tag_controller = biz::image_tag::Controller::new(img_tag_repo, fs_entry_repo.clone());
    *state.image_tag_controller.lock().unwrap() = Some(img_tag_controller);

    // Initialize thumbnail process
    let proc_th_queue = th_queue.clone();
    tokio::spawn(async move { queue::queue_proc(proc_th_queue, fs_entry_repo).await });

    Ok(())
}
