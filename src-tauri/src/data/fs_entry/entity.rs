use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString, Serialize, PartialEq, Clone)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Debug, Display, EnumString, Serialize, PartialEq, Clone)]
pub enum ImageType {
    #[strum(serialize = "")]
    None,

    #[strum(serialize = "jpg", serialize = "jpeg")]
    Jpeg,

    #[strum(serialize = "png")]
    Png,

    #[strum(serialize = "bmp")]
    Bmp,

    #[strum(serialize = "tga")]
    Tga,
}

pub fn parse_image_type(s: String) -> ImageType {
    match ImageType::from_str(&s) {
        Ok(i) => i,
        Err(_e) => ImageType::None,
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct FsEntryIds(pub String, pub String);

#[derive(Debug, Serialize, Clone)]
pub struct FsEntry {
    pub relative_path: String,
    pub base_path: String,
    pub source_id: String,
    pub fs_type: FileType,
    pub hidden: bool,
    pub sha256: String,
    pub image_type: ImageType,
    pub thumbnail_path: String,
    pub thumbnail_generating: bool,
    pub additional_fields: Vec<AdditionalField>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdditionalField {
    pub label: String,
    pub value: String,
}

pub struct DbFsEntry {
    pub relative_path: String,
    pub base_path: String,
    pub source_id: String,
    pub fs_type: String,
    pub hidden: bool,
    pub sha256: String,
    pub image_type: String,
    pub thumbnail_path: String,
    pub thumbnail_generating: bool,
    pub additional_fields: String,
}

// Implement mapping methods
impl From<FsEntry> for DbFsEntry {
    fn from(e: FsEntry) -> Self {
        let additional_fields = serde_json::to_string(&e.additional_fields).unwrap();

        Self {
            relative_path: e.relative_path,
            base_path: e.base_path,
            source_id: e.source_id,
            fs_type: e.fs_type.to_string(),
            hidden: e.hidden,
            sha256: e.sha256,
            image_type: e.image_type.to_string(),
            thumbnail_path: e.thumbnail_path,
            thumbnail_generating: e.thumbnail_generating,
            additional_fields,
        }
    }
}

impl From<DbFsEntry> for FsEntry {
    fn from(e: DbFsEntry) -> Self {
        let additional_fields: Vec<AdditionalField> =
            serde_json::from_str(&e.additional_fields).unwrap();

        Self {
            relative_path: e.relative_path,
            base_path: e.base_path,
            source_id: e.source_id,
            fs_type: FileType::from_str(&e.fs_type).unwrap(),
            hidden: e.hidden,
            sha256: e.sha256,
            image_type: ImageType::from_str(&e.image_type).unwrap(),
            thumbnail_path: e.thumbnail_path,
            thumbnail_generating: e.thumbnail_generating,
            additional_fields,
        }
    }
}
