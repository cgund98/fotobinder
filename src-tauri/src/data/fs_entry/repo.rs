use std::sync::Arc;

use super::entity;

pub struct Repo {
    pool: Arc<crate::state::PoolType>,
}

impl Repo {
    pub fn new(pool: Arc<crate::state::PoolType>) -> Repo {
        Repo { pool }
    }

    pub fn save(&self, entry: entity::FsEntry) -> Result<entity::FsEntry, crate::errors::AppError> {
        let d_entry = entity::DbFsEntry::from(entry);

        self.pool.get().unwrap().execute(
            "INSERT INTO fs_entries \
                (name, subpath, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10) \
            ON CONFLICT(name, subpath, source_id) DO UPDATE SET \
                fs_type=excluded.fs_type, \
                hidden=excluded.hidden, \
                sha256=excluded.sha256, \
                image_type=excluded.image_type, \
                thumbnail_path=excluded.thumbnail_path, \
                thumbnail_generating=excluded.thumbnail_generating, \
                additional_fields=excluded.additional_fields",
            (
                &d_entry.name,
                &d_entry.subpath,
                &d_entry.source_id,
                &d_entry.fs_type,
                &d_entry.hidden,
                &d_entry.sha256,
                &d_entry.image_type,
                &d_entry.thumbnail_path,
                &d_entry.thumbnail_generating,
                &d_entry.additional_fields,
            ),
        )?;

        let new_source = self.get_by_ids(&d_entry.name, &d_entry.subpath, &d_entry.source_id)?;

        Ok(new_source)
    }

    pub fn get_by_ids(
        &self,
        name: &str,
        subpath: &str,
        source_id: &str,
    ) -> Result<entity::FsEntry, crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        let mut stmt = pool.prepare(
            "SELECT \
                name, subpath, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields \
            FROM fs_entries \
            WHERE \
                name = ?1 AND \
                subpath = ?2 AND \
                source_id = ?3",
        )?;

        // Map results
        let ent_iter = stmt.query_map([name, subpath, source_id], |row| {
            Ok(entity::DbFsEntry {
                name: row.get(0)?,
                subpath: row.get(1)?,
                source_id: row.get(2)?,
                fs_type: row.get(3)?,
                hidden: row.get(4)?,
                sha256: row.get(5)?,
                image_type: row.get(6)?,
                thumbnail_path: row.get(7)?,
                thumbnail_generating: row.get(8)?,
                additional_fields: row.get(9)?,
            })
        })?;

        match ent_iter.last() {
            Some(ent) => Ok(entity::FsEntry::from(ent.unwrap())),
            None => Err(crate::errors::AppError::NotFound(format!(
                "name = '{}', subpath = '{}', source_id = {}",
                name, subpath, source_id
            ))),
        }
    }

    pub fn list_by_source_id_and_path_prefix(
        &self,
        source_id: &str,
        path_prefix: &str,
    ) -> Result<Vec<entity::FsEntry>, crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        let mut stmt =
            pool.prepare("SELECT \
            name, subpath, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields \
        FROM fs_entries \
        WHERE \
            source_id = ?1 AND
            subpath = ?2 \
            ")?;

        // Map results
        let ent_iter = stmt.query_map([source_id, path_prefix], |row| {
            Ok(entity::DbFsEntry {
                name: row.get(0)?,
                subpath: row.get(1)?,
                source_id: row.get(2)?,
                fs_type: row.get(3)?,
                hidden: row.get(4)?,
                sha256: row.get(5)?,
                image_type: row.get(6)?,
                thumbnail_path: row.get(7)?,
                thumbnail_generating: row.get(8)?,
                additional_fields: row.get(9)?,
            })
        })?;

        let mut entries: Vec<entity::FsEntry> = Vec::new();

        for db_entry in ent_iter {
            let entry = entity::FsEntry::from(db_entry.unwrap());

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list_by_source_id(
        &self,
        source_id: &str,
    ) -> Result<Vec<entity::FsEntry>, crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        let mut stmt =
            pool.prepare("SELECT \
            name, subpath, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields \
        FROM fs_entries \
        WHERE \
            source_id = ?1")?;

        // Map results
        let ent_iter = stmt.query_map([source_id], |row| {
            Ok(entity::DbFsEntry {
                name: row.get(0)?,
                subpath: row.get(1)?,
                source_id: row.get(2)?,
                fs_type: row.get(3)?,
                hidden: row.get(4)?,
                sha256: row.get(5)?,
                image_type: row.get(6)?,
                thumbnail_path: row.get(7)?,
                thumbnail_generating: row.get(8)?,
                additional_fields: row.get(9)?,
            })
        })?;

        let mut entries: Vec<entity::FsEntry> = Vec::new();

        for db_entry in ent_iter {
            let entry = entity::FsEntry::from(db_entry.unwrap());

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list_by_source_id_and_missing_thumbnails(
        &self,
        source_id: &str,
    ) -> Result<Vec<entity::FsEntry>, crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        let mut stmt =
            pool.prepare("SELECT \
            name, subpath, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields \
        FROM fs_entries \
        WHERE \
            source_id = ?1 \
            AND fs_type = 'File' \
            AND thumbnail_generating = true")?;

        // Map results
        let ent_iter = stmt.query_map([source_id], |row| {
            Ok(entity::DbFsEntry {
                name: row.get(0)?,
                subpath: row.get(1)?,
                source_id: row.get(2)?,
                fs_type: row.get(3)?,
                hidden: row.get(4)?,
                sha256: row.get(5)?,
                image_type: row.get(6)?,
                thumbnail_path: row.get(7)?,
                thumbnail_generating: row.get(8)?,
                additional_fields: row.get(9)?,
            })
        })?;

        let mut entries: Vec<entity::FsEntry> = Vec::new();

        for db_entry in ent_iter {
            let entry = entity::FsEntry::from(db_entry.unwrap());

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn delete(
        &self,
        name: &str,
        subpath: &str,
        source_id: &str,
    ) -> Result<(), crate::errors::AppError> {
        let pool = self.pool.get().unwrap();
        pool.execute(
            "DELETE FROM fs_entries WHERE \
                name = ?1 AND \
                subpath = ?2 AND \
                source_id = ?3",
            [name, subpath, source_id],
        )?;

        Ok(())
    }
}
