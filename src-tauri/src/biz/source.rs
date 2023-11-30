use uuid::Uuid;

use crate::{
    data::source::{entity, repo::Repo},
    errors::app,
};

pub struct Controller {
    repo: Repo,
}

pub fn new_controller(repo: Repo) -> Controller {
    Controller { repo }
}

impl Controller {
    pub fn get_by_id(&self, id: String) -> Result<entity::Source, app::Error> {
        self.repo.get_by_id(&id)
    }

    pub fn create(&self, name: &str) -> Result<entity::Source, app::Error> {
        let source = entity::Source {
            id: Uuid::new_v4().to_string(),
            source_type: entity::SourceType::Local,
            name: String::from(name),
            path: String::from("/my/path"),
            synced_at: chrono::Utc::now(),
        };

        self.repo.save(source)
    }
}
