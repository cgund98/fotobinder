#[derive(serde::Serialize, Clone)]
pub struct ImageTag {
    pub relative_path: String,
    pub source_id: String,
    pub tag_id: String,
}

pub type DbImageTag = ImageTag;
