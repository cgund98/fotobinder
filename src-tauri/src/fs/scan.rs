use walkdir::WalkDir;

use crate::{data::fs_entry::entity, errors::AppError};

#[derive(Debug)]
pub struct ScanEntry {
    pub depth: usize,
    pub name: String,
    pub is_dir: bool,
    pub ext: entity::ImageType,
    pub subpath: String,
}
// Build a list of all scanned entries
pub fn scan_directory(root_dir: &str) -> Result<Vec<ScanEntry>, AppError> {
    let mut entries: Vec<ScanEntry> = Vec::new();
    for entry in WalkDir::new(root_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let path = entry.path();
        let ext = match path.extension() {
            Some(s) => entity::parse_image_type(String::from(s.to_string_lossy())),
            None => entity::ImageType::None,
        };

        // Only allow entries that are either a directory or a valid image file
        if !path.is_dir() && ext == entity::ImageType::None {
            continue;
        }

        let mut e = ScanEntry {
            depth: entry.depth(),
            name: String::from(f_name),
            is_dir: path.is_dir(),
            subpath: String::from(""),
            ext,
        };

        if e.depth > 0 {
            let parent_path = path.parent().unwrap();
            let subpath = parent_path.strip_prefix(root_dir)?;
            e.subpath = String::from(subpath.to_string_lossy());
        }

        entries.push(e);
    }

    // Sort by depth to ensure we process directories before their children
    entries.sort_by(|a, b| a.depth.cmp(&b.depth));

    Ok(entries)
}
