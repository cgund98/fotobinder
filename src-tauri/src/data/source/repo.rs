use super::entity;
use crate::errors;

pub struct Repo {
    pool: crate::state::PoolType,
}

pub fn new(pool: crate::state::PoolType) -> Repo {
    Repo { pool }
}

impl Repo {
    pub fn save(&self, source: entity::Source) -> Result<entity::Source, errors::app::Error> {
        let d_source = entity::DbSource::from(source);

        self.pool.get().unwrap().execute(
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

    pub fn get_by_id(&self, id: &str) -> Result<entity::Source, errors::app::Error> {
        let pool = self.pool.get().unwrap();
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
            Some(ent) => Ok(entity::Source::from(ent.unwrap())),
            None => Err(errors::app::not_found()),
        }
    }
}
