use std::str::FromStr;

use uuid::Uuid;

use crate::data::source::{entity, repo::Repo};
use crate::errors::AppError;

pub struct Controller {
    repo: Repo,
}

impl Controller {
    pub fn new(repo: Repo) -> Controller {
        Controller { repo }
    }

    pub fn validate_payload(
        &self,
        name: &str,
        source_type: &str,
        path: &str,
    ) -> Result<(), AppError> {
        // Validate name
        if name.is_empty() {
            return Err(AppError::BadRequest(String::from(
                "Name must not be empty.",
            )));
        }

        // Validate type
        let source_type_err = match entity::SourceType::from_str(source_type) {
            Ok(_value) => None,
            Err(_err) => Some(AppError::BadRequest(String::from(
                "Bad source type. Unable to convert to enum.",
            ))),
        };

        if let Some(src_err) = source_type_err {
            return Err(src_err);
        }

        // Check path is a valid path
        if path.is_empty() {
            return Err(AppError::BadRequest(String::from(
                "Path must not be empty.",
            )));
        }

        if !std::path::Path::new(path).try_exists().unwrap() {
            return Err(AppError::BadRequest(String::from("Path does not exist.")));
        }

        match self.repo.get_by_path(path) {
            Ok(_res) => Err(AppError::BadRequest(String::from(
                "Resource already exists with that path.",
            ))),
            Err(AppError::NotFound(_)) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn create(
        &self,
        name: &str,
        source_type: &str,
        path: &str,
    ) -> Result<entity::Source, AppError> {
        self.validate_payload(name, source_type, path)?;

        let source = entity::Source {
            id: Uuid::new_v4().to_string(),
            source_type: entity::SourceType::from_str(source_type).unwrap(),
            name: String::from(name),
            path: String::from(path),
            synced_at: chrono::Utc::now(),
        };

        self.repo.save(source)
    }

    pub fn get_by_id(&self, id: &str) -> Result<entity::Source, AppError> {
        self.repo.get_by_id(id)
    }

    pub fn update_by_id(&self, id: &str, name: &str) -> Result<entity::Source, AppError> {
        let mut source = self.repo.get_by_id(id)?;

        source.name = String::from(name);

        self.repo.save(source)
    }

    pub fn list(&self) -> Result<Vec<entity::Source>, AppError> {
        self.repo.list()
    }

    pub fn delete(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete(id)
    }
}
