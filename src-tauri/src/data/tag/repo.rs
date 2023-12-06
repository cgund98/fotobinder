use std::sync::Arc;

use super::entity;

pub struct Repo {
    pool: Arc<crate::state::PoolType>,
}

impl Repo {
    pub fn new(pool: Arc<crate::state::PoolType>) -> Repo {
        Repo { pool }
    }

    pub fn save(&self, entry: entity::Tag) -> Result<entity::Tag, crate::errors::AppError> {
        let d_entry = entity::DbTag::from(entry);

        self.pool.get().unwrap().execute(
            "INSERT INTO tags (id, parent_id, name) VALUES (?1, ?2, ?3) \
            ON CONFLICT(id) DO UPDATE SET \
                parent_id=excluded.parent_id, \
                name=excluded.name",
            (&d_entry.id, &d_entry.parent_id, &d_entry.name),
        )?;

        let new_source = self.get_by_id(&d_entry.id)?;

        Ok(new_source)
    }

    pub fn get_by_id(&self, id: &str) -> Result<entity::Tag, crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        let mut stmt = pool.prepare("SELECT id, parent_id, name FROM tags WHERE id = ?1")?;

        // Map results
        let ent_iter = stmt.query_map([id], |row| {
            Ok(entity::DbTag {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                name: row.get(2)?,
            })
        })?;

        match ent_iter.last() {
            Some(ent) => Ok(ent.unwrap()),
            None => Err(crate::errors::AppError::NotFound(format!("id = '{}'", id,))),
        }
    }

    pub fn list(&self) -> Result<Vec<entity::Tag>, crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        let mut stmt = pool.prepare("SELECT id, parent_id, name FROM tags")?;

        // Map results
        let ent_iter = stmt.query_map([], |row| {
            Ok(entity::DbTag {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                name: row.get(2)?,
            })
        })?;

        let mut entries: Vec<entity::Tag> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry.unwrap();

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn delete(&self, id: &str) -> Result<(), crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        pool.execute("DELETE FROM tags WHERE id = ?1", [id])?;

        Ok(())
    }
}
