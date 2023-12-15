#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Collection {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
}

pub type DbCollection = Collection;
