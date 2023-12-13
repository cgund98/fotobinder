use std::sync::Arc;

use super::entity;

pub struct Repo {
    pool: Arc<crate::state::PoolType>,
}

impl Repo {
    pub fn new(pool: Arc<crate::state::PoolType>) -> Repo {
        Repo { pool }
    }

    pub fn save(&self, entry: entity::PathTag) -> Result<entity::PathTag, crate::errors::AppError> {
        let d_entry = entity::DbPathTag::from(entry);

        let p = self.pool.get()?;
        p.execute(
            "INSERT INTO path_tags (tag_id, base_path, source_id) VALUES (?1, ?2, ?3) \
            ON CONFLICT(tag_id, base_path, source_id) DO NOTHING",
            (&d_entry.tag_id, &d_entry.base_path, &d_entry.source_id),
        )?;

        let new_source =
            self.get_by_ids(&d_entry.tag_id, &d_entry.base_path, &d_entry.source_id)?;

        Ok(new_source)
    }

    pub fn get_by_ids(
        &self,
        tag_id: &str,
        base_path: &str,
        source_id: &str,
    ) -> Result<entity::PathTag, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare(
            "SELECT tag_id, base_path, source_id FROM path_tags \
                  WHERE tag_id = ?1 AND base_path = ?2 AND source_id = ?3",
        )?;

        // Map results
        let ent_iter = stmt.query_map([tag_id, base_path, source_id], |row| {
            Ok(entity::DbPathTag {
                tag_id: row.get(0)?,
                base_path: row.get(1)?,
                source_id: row.get(2)?,
            })
        })?;

        match ent_iter.last() {
            Some(ent) => Ok(ent?),
            None => Err(crate::errors::AppError::NotFound(format!(
                "tag_id = '{}', base_path = '{}', source_id = '{}'",
                tag_id, base_path, source_id,
            ))),
        }
    }

    pub fn list_by_base_path(
        &self,
        base_path: &str,
        source_id: &str,
    ) -> Result<Vec<entity::PathTag>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare(
            "SELECT tag_id, base_path, source_id FROM path_tags \
                  WHERE base_path = ?1 AND source_id = ?2",
        )?;

        // Map results
        let ent_iter = stmt.query_map([base_path, source_id], |row| {
            Ok(entity::DbPathTag {
                tag_id: row.get(0)?,
                base_path: row.get(1)?,
                source_id: row.get(2)?,
            })
        })?;

        let mut entries: Vec<entity::PathTag> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry?;

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list(&self) -> Result<Vec<entity::PathTag>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare("SELECT tag_id, base_path, source_id FROM path_tags")?;

        // Map results
        let ent_iter = stmt.query_map([], |row| {
            Ok(entity::DbPathTag {
                tag_id: row.get(0)?,
                base_path: row.get(1)?,
                source_id: row.get(2)?,
            })
        })?;

        let mut entries: Vec<entity::PathTag> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry?;

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn delete(
        &self,
        tag_id: &str,
        base_path: &str,
        source_id: &str,
    ) -> Result<(), crate::errors::AppError> {
        let pool = self.pool.get()?;
        pool.execute(
            "DELETE FROM path_tags WHERE tag_id = ?1 AND base_path = ?2 AND source_id = ?3",
            [tag_id, base_path, source_id],
        )?;

        Ok(())
    }
}
