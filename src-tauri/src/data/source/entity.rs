use serde::Serialize;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Serialize)]
pub enum SourceType {
    Local,
}

pub struct Source {
    pub id: String,
    pub source_type: SourceType,
    pub name: String,
    pub path: String,
    pub synced_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct DbSource {
    pub id: String,
    pub source_type: String,
    pub name: String,
    pub path: String,
    pub synced_at: String,
}

// Implement mapping methods
impl From<Source> for DbSource {
    fn from(e: Source) -> Self {
        let synced_at = e.synced_at.to_rfc2822();

        Self {
            id: e.id,
            source_type: e.source_type.to_string(),
            name: e.name,
            path: e.path,
            synced_at,
        }
    }
}

impl From<DbSource> for Source {
    fn from(e: DbSource) -> Self {
        let synced_at = chrono::DateTime::parse_from_rfc2822(&e.synced_at)
            .unwrap()
            .with_timezone(&chrono::Utc);

        Self {
            id: e.id,
            source_type: SourceType::from_str(&e.source_type).unwrap(),
            name: e.name,
            path: e.path,
            synced_at,
        }
    }
}
