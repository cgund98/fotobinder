#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Tag {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
}

pub type DbTag = Tag;
