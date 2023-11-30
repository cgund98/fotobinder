use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Serialize)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Display, EnumString, Serialize)]
pub enum ImageType {
    Jpeg,
    Png,
}

#[derive(Serialize)]
pub struct FsEntry {
    pub path: String,
    pub source: String,
    pub fs_type: FileType,
    pub hidden: bool,
    pub sha256: String,
    pub image_type: ImageType,
    pub thumbnail_path: String,
    pub additional_fields: Vec<AdditionalField>,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionalField {
    pub label: String,
    pub value: String,
}

pub struct DbFsEntry {
    pub path: String,
    pub source: String,
    pub fs_type: String,
    pub hidden: bool,
    pub sha256: String,
    pub image_type: String,
    pub thumbnail_path: String,
    pub additional_fields: String,
}

// Implement mapping methods
impl From<FsEntry> for DbFsEntry {
    fn from(e: FsEntry) -> Self {
        let additional_fields = serde_json::to_string(&e.additional_fields).unwrap();

        Self {
            path: e.path,
            source: e.source,
            fs_type: e.fs_type.to_string(),
            hidden: e.hidden,
            sha256: e.sha256,
            image_type: e.image_type.to_string(),
            thumbnail_path: e.thumbnail_path,
            additional_fields,
        }
    }
}

impl From<DbFsEntry> for FsEntry {
    fn from(e: DbFsEntry) -> Self {
        let additional_fields: Vec<AdditionalField> =
            serde_json::from_str(&e.additional_fields).unwrap();

        Self {
            path: e.path,
            source: e.source,
            fs_type: FileType::from_str(&e.fs_type).unwrap(),
            hidden: e.hidden,
            sha256: e.sha256,
            image_type: ImageType::from_str(&e.image_type).unwrap(),
            thumbnail_path: e.thumbnail_path,
            additional_fields,
        }
    }
}
