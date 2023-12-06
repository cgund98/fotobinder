#[derive(serde::Serialize, Clone)]
pub struct PathTag {
    pub base_path: String,
    pub source_id: String,
    pub tag_id: String,
}

pub type DbPathTag = PathTag;
