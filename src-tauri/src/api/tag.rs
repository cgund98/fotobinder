use crate::data::tag::entity;

#[derive(serde::Serialize)]
pub struct Tags {
    pub tags: Vec<entity::Tag>,
}

impl From<Vec<entity::Tag>> for Tags {
    fn from(e: Vec<entity::Tag>) -> Self {
        Self { tags: e }
    }
}
