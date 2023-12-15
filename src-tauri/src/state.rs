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
    pub fs_entry_controller: Mutex<Option<biz::fs_entry::Controller>>,
    pub tag_controller: Mutex<Option<biz::tag::Controller>>,
    pub path_tag_controller: Mutex<Option<biz::path_tag::Controller>>,
    pub image_tag_controller: Mutex<Option<biz::image_tag::Controller>>,
    pub collection_controller: Mutex<Option<biz::collection::Controller>>,
    pub collection_image_controller: Mutex<Option<biz::collection_image::Controller>>,
}

// Implement convenience traits
pub trait ServiceAccess {
    fn source_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::source::Controller) -> TResult;

    fn fs_entry_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::fs_entry::Controller) -> TResult;

    fn tag_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::tag::Controller) -> TResult;

    fn path_tag_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::path_tag::Controller) -> TResult;

    fn image_tag_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::image_tag::Controller) -> TResult;

    fn collection_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::collection::Controller) -> TResult;

    fn collection_image_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::collection_image::Controller) -> TResult;
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

    fn fs_entry_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::fs_entry::Controller) -> TResult,
    {
        let app_state: State<AppState> = tauri::Manager::state(self);
        let binding = app_state.fs_entry_controller.lock().unwrap();
        let ctrl = binding.as_ref().unwrap();

        operation(ctrl)
    }

    fn tag_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::tag::Controller) -> TResult,
    {
        let app_state: State<AppState> = tauri::Manager::state(self);
        let binding = app_state.tag_controller.lock().unwrap();
        let ctrl = binding.as_ref().unwrap();

        operation(ctrl)
    }

    fn path_tag_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::path_tag::Controller) -> TResult,
    {
        let app_state: State<AppState> = tauri::Manager::state(self);
        let binding = app_state.path_tag_controller.lock().unwrap();
        let ctrl = binding.as_ref().unwrap();

        operation(ctrl)
    }

    fn image_tag_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::image_tag::Controller) -> TResult,
    {
        let app_state: State<AppState> = tauri::Manager::state(self);
        let binding = app_state.image_tag_controller.lock().unwrap();
        let ctrl = binding.as_ref().unwrap();

        operation(ctrl)
    }

    fn collection_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::collection::Controller) -> TResult,
    {
        let app_state: State<AppState> = tauri::Manager::state(self);
        let binding = app_state.collection_controller.lock().unwrap();
        let ctrl = binding.as_ref().unwrap();

        operation(ctrl)
    }

    fn collection_image_ctrl<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&biz::collection_image::Controller) -> TResult,
    {
        let app_state: State<AppState> = tauri::Manager::state(self);
        let binding = app_state.collection_image_controller.lock().unwrap();
        let ctrl = binding.as_ref().unwrap();

        operation(ctrl)
    }
}
