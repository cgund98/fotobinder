use std::sync::Arc;

use crate::{
    api,
    data::{
        fs_entry,
        image_tag::{entity, repo::Repo},
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
        relative_path: &str,
        source_id: &str,
    ) -> Result<entity::ImageTag, AppError> {
        // Create new tag
        let tag = entity::ImageTag {
            tag_id: String::from(tag_id),
            relative_path: String::from(relative_path),
            source_id: String::from(source_id),
        };

        // Ensure directory exists
        self.fs_repo.get_by_ids(relative_path, source_id)?;

        self.repo.save(tag)
    }

    pub fn list(&self) -> Result<Vec<entity::ImageTag>, AppError> {
        self.repo.list()
    }

    pub fn get_by_ids(
        &self,
        tag_id: &str,
        relative_path: &str,
        source_id: &str,
    ) -> Result<entity::ImageTag, AppError> {
        self.repo.get_by_ids(tag_id, relative_path, source_id)
    }

    pub fn list_by_relative_path(
        &self,
        relative_path: &str,
        source_id: &str,
    ) -> Result<Vec<entity::ImageTag>, AppError> {
        self.repo.list_by_relative_path(relative_path, source_id)
    }

    pub fn delete(
        &self,
        tag_id: &str,
        relative_path: &str,
        source_id: &str,
    ) -> Result<(), AppError> {
        self.repo.delete(tag_id, relative_path, source_id)
    }

    pub fn assign(
        &self,
        relative_paths: Vec<String>,
        source_ids: Vec<String>,
        assignments: api::image_tag::TagAssignments,
    ) -> Result<(), AppError> {
        for (idx, relative_path) in relative_paths.iter().enumerate() {
            let source_id = source_ids.get(idx).unwrap();

            // Ensure path exists
            self.fs_repo.get_by_ids(relative_path, source_id)?;

            // Add tags
            for tag_id in assignments.add.iter() {
                let tag = entity::ImageTag {
                    relative_path: String::from(relative_path),
                    source_id: String::from(source_id),
                    tag_id: String::from(tag_id),
                };
                self.repo.save(tag)?;
            }

            // Remove tags
            for tag_id in assignments.remove.iter() {
                self.repo.delete(tag_id, relative_path, source_id)?;
            }
        }

        Ok(())
    }
}
