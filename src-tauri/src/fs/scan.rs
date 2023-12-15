use walkdir::WalkDir;

use crate::{data::fs_entry::entity, errors::AppError};

#[derive(Debug)]
pub struct ScanEntry {
    pub depth: usize,
    pub relative_path: String,
    pub is_dir: bool,
    pub ext: entity::ImageType,
    pub base_path: String,
}
// Build a list of all scanned entries
pub fn scan_directory(root_dir: &str) -> Result<Vec<ScanEntry>, AppError> {
    let mut entries: Vec<ScanEntry> = Vec::new();

    for entry in WalkDir::new(root_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let ext = match path.extension() {
            Some(s) => entity::parse_image_type(String::from(s.to_string_lossy())),
            None => entity::ImageType::None,
        };

        // Only allow entries that are either a directory or a valid image file
        let is_root_dir = path.to_string_lossy().eq(root_dir);
        let is_invalid_image_file = !path.is_dir() && ext == entity::ImageType::None;
        if is_root_dir || is_invalid_image_file {
            continue;
        }

        if entry.depth() == 0 {
            continue;
        }

        let relative_path = path.strip_prefix(root_dir)?;
        let parent_path = path.parent().unwrap();
        let base_path = parent_path.strip_prefix(root_dir)?;

        let e = ScanEntry {
            depth: entry.depth(),
            relative_path: String::from(relative_path.to_string_lossy()),
            is_dir: path.is_dir(),
            base_path: String::from(base_path.to_string_lossy()),
            ext,
        };

        entries.push(e);
    }

    // Sort by depth to ensure we process directories before their children
    entries.sort_by(|a, b| a.depth.cmp(&b.depth));

    Ok(entries)
}
