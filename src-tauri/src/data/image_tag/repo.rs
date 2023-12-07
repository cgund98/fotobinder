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
        entry: entity::ImageTag,
    ) -> Result<entity::ImageTag, crate::errors::AppError> {
        let d_entry = entity::DbImageTag::from(entry);

        self.pool.get().unwrap().execute(
            "INSERT INTO image_tags (tag_id, relative_path, source_id) VALUES (?1, ?2, ?3) \
            ON CONFLICT(tag_id, relative_path, source_id) DO NOTHING",
            (&d_entry.tag_id, &d_entry.relative_path, &d_entry.source_id),
        )?;

        let new_source =
            self.get_by_ids(&d_entry.tag_id, &d_entry.relative_path, &d_entry.source_id)?;

        Ok(new_source)
    }

    pub fn get_by_ids(
        &self,
        tag_id: &str,
        relative_path: &str,
        source_id: &str,
    ) -> Result<entity::ImageTag, crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        let mut stmt = pool.prepare(
            "SELECT tag_id, relative_path, source_id FROM image_tags \
                  WHERE tag_id = ?1 AND relative_path = ?2 AND source_id = ?3",
        )?;

        // Map results
        let ent_iter = stmt.query_map([tag_id, relative_path, source_id], |row| {
            Ok(entity::DbImageTag {
                tag_id: row.get(0)?,
                relative_path: row.get(1)?,
                source_id: row.get(2)?,
            })
        })?;

        match ent_iter.last() {
            Some(ent) => Ok(ent.unwrap()),
            None => Err(crate::errors::AppError::NotFound(format!(
                "tag_id = '{}', relative_path = '{}', source_id = '{}'",
                tag_id, relative_path, source_id,
            ))),
        }
    }

    pub fn list_by_relative_path(
        &self,
        relative_path: &str,
        source_id: &str,
    ) -> Result<Vec<entity::ImageTag>, crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        let mut stmt = pool.prepare(
            "SELECT tag_id, relative_path, source_id FROM image_tags \
                  WHERE relative_path = ?1 AND source_id = ?2",
        )?;

        // Map results
        let ent_iter = stmt.query_map([relative_path, source_id], |row| {
            Ok(entity::DbImageTag {
                tag_id: row.get(0)?,
                relative_path: row.get(1)?,
                source_id: row.get(2)?,
            })
        })?;

        let mut entries: Vec<entity::ImageTag> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry.unwrap();

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list(&self) -> Result<Vec<entity::ImageTag>, crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        let mut stmt = pool.prepare("SELECT tag_id, relative_path, source_id FROM image_tags")?;

        // Map results
        let ent_iter = stmt.query_map([], |row| {
            Ok(entity::DbImageTag {
                tag_id: row.get(0)?,
                relative_path: row.get(1)?,
                source_id: row.get(2)?,
            })
        })?;

        let mut entries: Vec<entity::ImageTag> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry.unwrap();

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn delete(
        &self,
        tag_id: &str,
        relative_path: &str,
        source_id: &str,
    ) -> Result<(), crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        pool.execute(
            "DELETE FROM image_tags WHERE tag_id = ?1 AND relative_path = ?2 AND source_id = ?3",
            [tag_id, relative_path, source_id],
        )?;

        Ok(())
    }
}
