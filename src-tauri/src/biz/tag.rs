use crate::{
    data::tag::{entity, repo::Repo},
    errors::AppError,
};

use uuid::Uuid;

pub struct Controller {
    repo: Repo,
}

impl Controller {
    pub fn new(repo: Repo) -> Controller {
        Controller { repo }
    }

    pub fn create(&self, name: &str, parent_id: Option<&str>) -> Result<entity::Tag, AppError> {
        // Create new tag
        let mut tag = entity::Tag {
            id: Uuid::new_v4().to_string(),
            parent_id: None,
            name: String::from(name),
        };

        if let Some(p) = parent_id {
            // Verify that parent exists
            self.repo.get_by_id(p)?;
            tag.parent_id = Some(String::from(p));
        }

        self.repo.save(tag)
    }

    pub fn list(&self) -> Result<Vec<entity::Tag>, AppError> {
        self.repo.list()
    }

    pub fn get_by_id(&self, id: &str) -> Result<entity::Tag, AppError> {
        self.repo.get_by_id(id)
    }

    pub fn update_by_id(
        &self,
        id: &str,
        name: &str,
        parent_id: Option<&str>,
    ) -> Result<entity::Tag, AppError> {
        // Fetch task
        let mut task = self.get_by_id(id)?;

        // Update fields
        task.name = String::from(name);
        if let Some(p) = parent_id {
            task.parent_id = Some(String::from(p));
        } else {
            task.parent_id = None;
        }

        self.repo.save(task)
    }

    pub fn delete(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete(id)
    }

    pub fn list_by_relative_path(
        &self,
        relative_path: &str,
        source_id: &str,
    ) -> Result<Vec<entity::Tag>, crate::errors::AppError> {
        self.repo.list_by_relative_path(relative_path, source_id)
    }
}
