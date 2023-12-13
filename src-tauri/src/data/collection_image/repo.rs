use std::sync::Arc;

use super::entity;

pub struct Repo {
    pool: Arc<crate::state::PoolType>,
}

impl Repo {
    pub fn new(pool: Arc<crate::state::PoolType>) -> Repo {
        Repo { pool }
    }

    pub fn save(
        &self,
        entry: entity::CollectionImage,
    ) -> Result<entity::CollectionImage, crate::errors::AppError> {
        let d_entry = entity::DbCollectionImage::from(entry);

        let p = self.pool.get()?;
        p.execute(
            "INSERT INTO collection_images (collection_id, relative_path, source_id) VALUES (?1, ?2, ?3) \
            ON CONFLICT(collection_id, relative_path, source_id) DO NOTHING",
            (&d_entry.collection_id, &d_entry.relative_path, &d_entry.source_id),
        )?;

        let new_source = self.get_by_ids(
            &d_entry.collection_id,
            &d_entry.relative_path,
            &d_entry.source_id,
        )?;

        Ok(new_source)
    }

    pub fn get_by_ids(
        &self,
        collection_id: &str,
        relative_path: &str,
        source_id: &str,
    ) -> Result<entity::CollectionImage, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare(
            "SELECT collection_id, relative_path, source_id FROM collection_images \
                  WHERE collection_id = ?1 AND relative_path = ?2 AND source_id = ?3",
        )?;

        // Map results
        let ent_iter = stmt.query_map([collection_id, relative_path, source_id], |row| {
            Ok(entity::DbCollectionImage {
                collection_id: row.get(0)?,
                relative_path: row.get(1)?,
                source_id: row.get(2)?,
            })
        })?;

        match ent_iter.last() {
            Some(ent) => Ok(ent?),
            None => Err(crate::errors::AppError::NotFound(format!(
                "collection_id = '{}', relative_path = '{}', source_id = '{}'",
                collection_id, relative_path, source_id,
            ))),
        }
    }

    pub fn list_by_relative_path(
        &self,
        relative_path: &str,
        source_id: &str,
    ) -> Result<Vec<entity::CollectionImage>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare(
            "SELECT collection_id, relative_path, source_id FROM collection_images \
                  WHERE relative_path = ?1 AND source_id = ?2",
        )?;

        // Map results
        let ent_iter = stmt.query_map([relative_path, source_id], |row| {
            Ok(entity::DbCollectionImage {
                collection_id: row.get(0)?,
                relative_path: row.get(1)?,
                source_id: row.get(2)?,
            })
        })?;

        let mut entries: Vec<entity::CollectionImage> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry?;

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list(&self) -> Result<Vec<entity::CollectionImage>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt =
            pool.prepare("SELECT collection_id, relative_path, source_id FROM collection_images")?;

        // Map results
        let ent_iter = stmt.query_map([], |row| {
            Ok(entity::DbCollectionImage {
                collection_id: row.get(0)?,
                relative_path: row.get(1)?,
                source_id: row.get(2)?,
            })
        })?;

        let mut entries: Vec<entity::CollectionImage> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry?;

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn delete(
        &self,
        collection_id: &str,
        relative_path: &str,
        source_id: &str,
    ) -> Result<(), crate::errors::AppError> {
        let pool = self.pool.get()?;
        pool.execute(
            "DELETE FROM collection_images WHERE collection_id = ?1 AND relative_path = ?2 AND source_id = ?3",
            [collection_id, relative_path, source_id],
        )?;

        Ok(())
    }
}
