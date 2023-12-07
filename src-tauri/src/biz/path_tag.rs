use std::sync::Arc;

use crate::{
    api,
    data::{
        fs_entry,
        path_tag::{entity, repo::Repo},
    },
    errors::AppError,
};

pub struct Controller {
    repo: Repo,
    fs_repo: Arc<fs_entry::repo::Repo>,
}

impl Controller {
    pub fn new(repo: Repo, fs_repo: Arc<fs_entry::repo::Repo>) -> Controller {
        Controller { repo, fs_repo }
    }

    pub fn create(
        &self,
        tag_id: &str,
        base_path: &str,
        source_id: &str,
    ) -> Result<entity::PathTag, AppError> {
        // Create new tag
        let tag = entity::PathTag {
            tag_id: String::from(tag_id),
            base_path: String::from(base_path),
            source_id: String::from(source_id),
        };

        // Ensure directory exists
        self.fs_repo.get_by_ids(base_path, source_id)?;

        self.repo.save(tag)
    }

    pub fn list(&self) -> Result<Vec<entity::PathTag>, AppError> {
        self.repo.list()
    }

    pub fn get_by_ids(
        &self,
        tag_id: &str,
        base_path: &str,
        source_id: &str,
    ) -> Result<entity::PathTag, AppError> {
        self.repo.get_by_ids(tag_id, base_path, source_id)
    }

    pub fn list_by_base_path(
        &self,
        base_path: &str,
        source_id: &str,
    ) -> Result<Vec<entity::PathTag>, AppError> {
        self.repo.list_by_base_path(base_path, source_id)
    }

    pub fn delete(&self, tag_id: &str, base_path: &str, source_id: &str) -> Result<(), AppError> {
        self.repo.delete(tag_id, base_path, source_id)
    }

    pub fn assign(
        &self,
        base_paths: Vec<String>,
        source_id: &str,
        assignments: api::image_tag::TagAssignments,
    ) -> Result<(), AppError> {
        for base_path in base_paths.iter() {
            // Ensure path exists
            self.fs_repo.get_by_ids(base_path, source_id)?;

            // Add tags
            for tag_id in assignments.add.iter() {
                let tag = entity::PathTag {
                    base_path: String::from(base_path),
                    source_id: String::from(source_id),
                    tag_id: String::from(tag_id),
                };
                self.repo.save(tag)?;
            }

            // Remove tags
            for tag_id in assignments.remove.iter() {
                self.repo.delete(tag_id, base_path, source_id)?;
            }
        }

        Ok(())
    }
}
