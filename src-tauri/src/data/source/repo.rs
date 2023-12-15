use std::sync::Arc;

use super::entity;

pub struct Repo {
    pool: Arc<crate::state::PoolType>,
}

impl Repo {
    pub fn new(pool: Arc<crate::state::PoolType>) -> Repo {
        Repo { pool }
    }

    pub fn save(&self, source: entity::Source) -> Result<entity::Source, crate::errors::AppError> {
        let d_source = entity::DbSource::from(source);

        let p = self.pool.get()?;
        p.execute(
            "INSERT INTO sources \
                (id, source_type, name, path, synced_at) VALUES (?1, ?2, ?3, ?4, ?5) \
            ON CONFLICT(id) DO UPDATE SET \
                source_type=excluded.source_type, \
                name=excluded.name, \
                path=excluded.path, \
                synced_at=excluded.synced_at",
            (
                &d_source.id,
                &d_source.source_type,
                &d_source.name,
                &d_source.path,
                &d_source.synced_at,
            ),
        )?;

        let new_source = self.get_by_id(&d_source.id)?;

        Ok(new_source)
    }

    pub fn get_by_id(&self, id: &str) -> Result<entity::Source, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool
            .prepare("SELECT id, source_type, name, path, synced_at FROM sources WHERE id = ?1")?;

        // Map results
        let ent_iter = stmt.query_map([&id], |row| {
            Ok(entity::DbSource {
                id: row.get(0)?,
                source_type: row.get(1)?,
                name: row.get(2)?,
                path: row.get(3)?,
                synced_at: row.get(4)?,
            })
        })?;

        match ent_iter.last() {
            Some(ent) => Ok(entity::Source::from(ent?)),
            None => Err(crate::errors::AppError::NotFound(format!("id = '{}'", id))),
        }
    }

    pub fn get_by_path(&self, path: &str) -> Result<entity::Source, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare(
            "SELECT id, source_type, name, path, synced_at FROM sources WHERE path = ?1",
        )?;

        // Map results
        let ent_iter = stmt.query_map([&path], |row| {
            Ok(entity::DbSource {
                id: row.get(0)?,
                source_type: row.get(1)?,
                name: row.get(2)?,
                path: row.get(3)?,
                synced_at: row.get(4)?,
            })
        })?;

        match ent_iter.last() {
            Some(ent) => Ok(entity::Source::from(ent?)),
            None => Err(crate::errors::AppError::NotFound(format!(
                "path = '{}'",
                path
            ))),
        }
    }

    pub fn list(&self) -> Result<Vec<entity::Source>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt =
            pool.prepare("SELECT id, source_type, name, path, synced_at FROM sources")?;

        // Map results
        let ent_iter = stmt.query_map([], |row| {
            Ok(entity::DbSource {
                id: row.get(0)?,
                source_type: row.get(1)?,
                name: row.get(2)?,
                path: row.get(3)?,
                synced_at: row.get(4)?,
            })
        })?;

        let mut sources: Vec<entity::Source> = Vec::new();

        for db_source in ent_iter {
            let source = entity::Source::from(db_source?);

            sources.push(source);
        }

        Ok(sources)
    }

    pub fn delete(&self, id: &str) -> Result<(), crate::errors::AppError> {
        self.pool
            .get()?
            .execute("DELETE FROM sources WHERE id = ?1", [id])?;

        Ok(())
    }
}
