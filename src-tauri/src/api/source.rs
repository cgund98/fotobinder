use serde::Serialize;

use crate::data::source::entity;

#[derive(Serialize)]
pub struct Source {
    pub id: String,
    pub source_type: String,
    pub name: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct Sources {
    pub sources: Vec<Source>,
}

// Implement mapping methods
impl From<entity::Source> for Source {
    fn from(e: entity::Source) -> Self {
        Self {
            id: e.id,
            source_type: e.source_type.to_string(),
            name: e.name,
            path: e.path,
        }
    }
}

impl From<Vec<entity::Source>> for Sources {
    fn from(e: Vec<entity::Source>) -> Self {
        let sources: Vec<Source> = e.into_iter().map(Source::from).collect();
        Self { sources }
    }
}
