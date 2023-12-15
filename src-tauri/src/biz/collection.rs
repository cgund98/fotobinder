use crate::{
    data::collection::{entity, repo::Repo},
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

    pub fn create(
        &self,
        name: &str,
        parent_id: Option<&str>,
    ) -> Result<entity::Collection, AppError> {
        // Create new collection
        let mut collection = entity::Collection {
            id: Uuid::new_v4().to_string(),
            parent_id: None,
            name: String::from(name),
        };

        if let Some(p) = parent_id {
            // Verify that parent exists
            self.repo.get_by_id(p)?;
            collection.parent_id = Some(String::from(p));
        }

        self.repo.save(collection)
    }

    pub fn list(&self) -> Result<Vec<entity::Collection>, AppError> {
        self.repo.list()
    }

    pub fn list_by_parent_id(
        &self,
        parent_id: Option<String>,
    ) -> Result<Vec<entity::Collection>, AppError> {
        self.repo.list_by_parent_id(parent_id)
    }

    pub fn get_by_id(&self, id: &str) -> Result<entity::Collection, AppError> {
        self.repo.get_by_id(id)
    }

    pub fn update_by_id(
        &self,
        id: &str,
        name: &str,
        parent_id: Option<&str>,
    ) -> Result<entity::Collection, AppError> {
        // Fetch collection
        let mut col = self.repo.get_by_id(id)?;

        // Update attributes
        col.name = String::from(name);
        if let Some(p) = parent_id {
            col.parent_id = Some(String::from(p));
        } else {
            col.parent_id = None;
        }

        // Save collection
        self.repo.save(col)
    }

    pub fn delete(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete(id)
    }
}
