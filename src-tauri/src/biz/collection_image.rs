use std::sync::Arc;

use crate::{
    data::{
        collection_image::{entity, repo::Repo},
        fs_entry,
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
        collection_id: &str,
        relative_path: &str,
        source_id: &str,
    ) -> Result<entity::CollectionImage, AppError> {
        // Create new collection
        let collection = entity::CollectionImage {
            collection_id: String::from(collection_id),
            relative_path: String::from(relative_path),
            source_id: String::from(source_id),
        };

        // Ensure directory exists
        self.fs_repo.get_by_ids(relative_path, source_id)?;

        self.repo.save(collection)
    }

    pub fn list(&self) -> Result<Vec<entity::CollectionImage>, AppError> {
        self.repo.list()
    }

    pub fn get_by_ids(
        &self,
        collection_id: &str,
        relative_path: &str,
        source_id: &str,
    ) -> Result<entity::CollectionImage, AppError> {
        self.repo
            .get_by_ids(collection_id, relative_path, source_id)
    }

    pub fn list_by_relative_path(
        &self,
        relative_path: &str,
        source_id: &str,
    ) -> Result<Vec<entity::CollectionImage>, AppError> {
        self.repo.list_by_relative_path(relative_path, source_id)
    }

    pub fn delete(
        &self,
        collection_id: &str,
        relative_path: &str,
        source_id: &str,
    ) -> Result<(), AppError> {
        self.repo.delete(collection_id, relative_path, source_id)?;

        Ok(())
    }

    pub fn delete_many(
        &self,
        collection_id: &str,
        relative_paths: Vec<String>,
        source_id: &str,
    ) -> Result<(), AppError> {
        for relative_path in relative_paths.iter() {
            self.repo.delete(collection_id, relative_path, source_id)?;
        }

        Ok(())
    }

    pub fn assign_many(
        &self,
        collection_id: &str,
        relative_paths: Vec<String>,
        source_ids: Vec<String>,
    ) -> Result<(), AppError> {
        for (idx, relative_path) in relative_paths.iter().enumerate() {
            let source_id = source_ids.get(idx).unwrap();
            // Ensure path exists
            self.fs_repo.get_by_ids(relative_path, source_id)?;

            // Add collections
            let collection = entity::CollectionImage {
                relative_path: String::from(relative_path),
                source_id: String::from(source_id),
                collection_id: String::from(collection_id),
            };
            self.repo.save(collection)?;
        }

        Ok(())
    }
}
