use serde::Serialize;

use crate::data::fs_entry::entity;

#[derive(Serialize)]
pub struct FsEntry {
    pub name: String,
    pub subpath: String,
    pub source_id: String,
    pub fs_type: entity::FileType,
    pub hidden: bool,
    pub image_type: entity::ImageType,
    pub thumbnail_path: String,
    pub thumbnail_generating: bool,
    pub additional_fields: Vec<entity::AdditionalField>,
}

#[derive(Serialize)]
pub struct FsEntries {
    pub entries: Vec<FsEntry>,
}

// Implement mapping methods
impl From<entity::FsEntry> for FsEntry {
    fn from(e: entity::FsEntry) -> Self {
        Self {
            name: e.name,
            subpath: e.subpath,
            source_id: e.source_id,
            fs_type: e.fs_type,
            hidden: e.hidden,
            image_type: e.image_type,
            thumbnail_path: e.thumbnail_path,
            thumbnail_generating: e.thumbnail_generating,
            additional_fields: e.additional_fields,
        }
    }
}

impl From<Vec<entity::FsEntry>> for FsEntries {
    fn from(e: Vec<entity::FsEntry>) -> Self {
        let entries: Vec<FsEntry> = e.into_iter().map(FsEntry::from).collect();
        Self { entries }
    }
}

#[derive(Serialize)]
pub struct ScanResults {
    pub entries_deleted: usize,
    pub entries_created: usize,
    pub thumbnails_created: usize,
}
