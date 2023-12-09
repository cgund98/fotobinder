use crate::data::collection::entity;

#[derive(serde::Serialize)]
pub struct Collections {
    pub collections: Vec<entity::Collection>,
}

impl From<Vec<entity::Collection>> for Collections {
    fn from(e: Vec<entity::Collection>) -> Self {
        Self { collections: e }
    }
}
