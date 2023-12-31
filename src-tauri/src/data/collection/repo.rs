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
        entry: entity::Collection,
    ) -> Result<entity::Collection, crate::errors::AppError> {
        let d_entry = entity::DbCollection::from(entry);

        let p = self.pool.get()?;
        p.execute(
            "INSERT INTO collections (id, parent_id, name) VALUES (?1, ?2, ?3) \
            ON CONFLICT(id) DO UPDATE SET \
                parent_id=excluded.parent_id, \
                name=excluded.name",
            (&d_entry.id, &d_entry.parent_id, &d_entry.name),
        )?;

        let new_source = self.get_by_id(&d_entry.id)?;

        Ok(new_source)
    }

    pub fn get_by_id(&self, id: &str) -> Result<entity::Collection, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare("SELECT id, parent_id, name FROM collections WHERE id = ?1")?;

        // Map results
        let ent_iter = stmt.query_map([id], |row| {
            Ok(entity::DbCollection {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                name: row.get(2)?,
            })
        })?;

        match ent_iter.last() {
            Some(ent) => Ok(ent?),
            None => Err(crate::errors::AppError::NotFound(format!("id = '{}'", id,))),
        }
    }

    pub fn list(&self) -> Result<Vec<entity::Collection>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare("SELECT id, parent_id, name FROM collections")?;

        // Map results
        let ent_iter = stmt.query_map([], |row| {
            Ok(entity::DbCollection {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                name: row.get(2)?,
            })
        })?;

        let mut entries: Vec<entity::Collection> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry?;

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list_by_parent_id(
        &self,
        parent_id: Option<String>,
    ) -> Result<Vec<entity::Collection>, crate::errors::AppError> {
        // Parse input variable
        let mut sql = "SELECT id, parent_id, name FROM collections WHERE parent_id ISNULL";
        let null_params = rusqlite::params![];
        let id_params = rusqlite::params![parent_id];
        let params: &[&dyn rusqlite::ToSql];

        if parent_id.clone().is_some() {
            sql = "SELECT id, parent_id, name FROM collections WHERE parent_id = ?1";
            params = id_params;
        } else {
            params = null_params;
        }

        let pool = self.pool.get()?;
        let mut stmt = pool.prepare(sql)?;

        // Map results
        let ent_iter = stmt.query_map(params, |row| {
            Ok(entity::DbCollection {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                name: row.get(2)?,
            })
        })?;

        let mut entries: Vec<entity::Collection> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry?;

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn delete(&self, id: &str) -> Result<(), crate::errors::AppError> {
        let pool = self.pool.get()?;
        pool.execute("DELETE FROM collections WHERE id = ?1", [id])?;

        Ok(())
    }
}
