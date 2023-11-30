use std::sync::Mutex;

use tauri::{AppHandle, State};

use r2d2_sqlite::SqliteConnectionManager;

use crate::biz;

pub type PoolType = r2d2::Pool<SqliteConnectionManager>;

pub type WrappedState<'a> = State<'a, AppState>;

// Define app state
#[derive(Default)]
pub struct AppState {
    pub pool: Mutex<Option<PoolType>>,
    pub source_controller: Mutex<Option<biz::source::Controller>>,
}

// Implement convenience traits
pub trait ServiceAccess {
    fn source_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::source::Controller) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn source_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::source::Controller) -> TResult,
    {
        let app_state: State<AppState> = tauri::Manager::state(self);
        let binding = app_state.source_controller.lock().unwrap();
        let ctrl = binding.as_ref().unwrap();

        operation(ctrl)
    }
}
