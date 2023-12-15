use crate::data::image_tag::entity;

#[derive(serde::Serialize)]
pub struct ImageTags {
    pub image_tags: Vec<entity::ImageTag>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TagAssignments {
    pub add: Vec<String>,
    pub remove: Vec<String>,
}

impl From<Vec<entity::ImageTag>> for ImageTags {
    fn from(e: Vec<entity::ImageTag>) -> Self {
        Self { image_tags: e }
    }
}
