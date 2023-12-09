// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fotobinder::cmd;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(fotobinder::state::AppState::default())
        .setup(fotobinder::init::init_deps)
        .invoke_handler(tauri::generate_handler![
            cmd::source::create_source,
            cmd::source::get_source,
            cmd::source::delete_source,
            cmd::source::list_sources,
            cmd::fs_entry::scan_source_dir,
            cmd::fs_entry::list_fs_entries_by_source_id,
            cmd::fs_entry::generate_missing_thumbnails,
            cmd::fs_entry::list_fs_entries_by_tags,
            cmd::fs_entry::list_fs_entries_by_collection_id,
            cmd::tag::create_tag,
            cmd::tag::get_tag,
            cmd::tag::delete_tag,
            cmd::tag::list_tags,
            cmd::path_tag::create_path_tag,
            cmd::path_tag::get_path_tag,
            cmd::path_tag::list_path_tags,
            cmd::path_tag::list_path_tags_by_base_path,
            cmd::path_tag::delete_path_tag,
            cmd::path_tag::assign_path_tags,
            cmd::image_tag::create_image_tag,
            cmd::image_tag::get_image_tag,
            cmd::image_tag::list_image_tags,
            cmd::image_tag::list_image_tags_by_relative_path,
            cmd::image_tag::delete_image_tag,
            cmd::image_tag::assign_image_tags,
            cmd::collection::create_collection,
            cmd::collection::get_collection,
            cmd::collection::list_collections,
            cmd::collection::list_collections_by_parent_id,
            cmd::collection::delete_collection,
            cmd::collection_image::create_collection_image,
            cmd::collection_image::get_collection_image,
            cmd::collection_image::list_collection_images,
            cmd::collection_image::list_collection_images_by_relative_path,
            cmd::collection_image::delete_collection_image,
            cmd::collection_image::delete_many_collection_images,
            cmd::collection_image::assign_many_collection_images,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
