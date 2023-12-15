use crate::data::path_tag::entity;

#[derive(serde::Serialize)]
pub struct PathTags {
    pub path_tags: Vec<entity::PathTag>,
}

impl From<Vec<entity::PathTag>> for PathTags {
    fn from(e: Vec<entity::PathTag>) -> Self {
        Self { path_tags: e }
    }
}
