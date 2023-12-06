use crate::data::image_tag::entity;

#[derive(serde::Serialize)]
pub struct ImageTags {
    pub image_tags: Vec<entity::ImageTag>,
}

impl From<Vec<entity::ImageTag>> for ImageTags {
    fn from(e: Vec<entity::ImageTag>) -> Self {
        Self { image_tags: e }
    }
}
