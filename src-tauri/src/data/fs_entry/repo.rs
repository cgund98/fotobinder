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

        let p = self.pool.get()?;
        p.execute(
            "INSERT INTO fs_entries \
                (relative_path, base_path, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10) \
            ON CONFLICT(relative_path, source_id) DO UPDATE SET \
                fs_type=excluded.fs_type, \
                hidden=excluded.hidden, \
                sha256=excluded.sha256, \
                image_type=excluded.image_type, \
                thumbnail_path=excluded.thumbnail_path, \
                thumbnail_generating=excluded.thumbnail_generating, \
                additional_fields=excluded.additional_fields",
            (
                &d_entry.relative_path,
                &d_entry.base_path,
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

        let new_source = self.get_by_ids(&d_entry.relative_path, &d_entry.source_id)?;

        Ok(new_source)
    }

    pub fn get_by_ids(
        &self,
        relative_path: &str,
        source_id: &str,
    ) -> Result<entity::FsEntry, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare(
            "SELECT \
                relative_path, base_path, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields \
            FROM fs_entries \
            WHERE \
                relative_path = ?1 AND \
                source_id = ?2",
        )?;

        // Map results
        let ent_iter = stmt.query_map([relative_path, source_id], |row| {
            Ok(entity::DbFsEntry {
                relative_path: row.get(0)?,
                base_path: row.get(1)?,
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
            Some(ent) => {
                let e = ent?;
                Ok(entity::FsEntry::from(e))
            }
            None => Err(crate::errors::AppError::NotFound(format!(
                "relative_path = '{}', source_id = {}",
                relative_path, source_id
            ))),
        }
    }

    pub fn list_by_source_id_and_path_prefix(
        &self,
        source_id: &str,
        path_prefix: &str,
    ) -> Result<Vec<entity::FsEntry>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt =
            pool.prepare("SELECT \
            relative_path, base_path, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields \
        FROM fs_entries \
        WHERE \
            source_id = ?1 AND
            base_path = ?2 \
            LIMIT 1000")?;

        // Map results
        let ent_iter = stmt.query_map([source_id, path_prefix], |row| {
            Ok(entity::DbFsEntry {
                relative_path: row.get(0)?,
                base_path: row.get(1)?,
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
            let d = db_entry?;
            let entry = entity::FsEntry::from(d);

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list_by_source_id(
        &self,
        source_id: &str,
    ) -> Result<Vec<entity::FsEntry>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt =
            pool.prepare("SELECT \
            relative_path, base_path, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields \
        FROM fs_entries \
        WHERE \
            source_id = ?1")?;

        // Map results
        let ent_iter = stmt.query_map([source_id], |row| {
            Ok(entity::DbFsEntry {
                relative_path: row.get(0)?,
                base_path: row.get(1)?,
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
            let d = db_entry?;
            let entry = entity::FsEntry::from(d);

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list_by_source_id_and_missing_thumbnails(
        &self,
        source_id: &str,
    ) -> Result<Vec<entity::FsEntry>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt =
            pool.prepare("SELECT \
            relative_path, base_path, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields \
        FROM fs_entries \
        WHERE \
            source_id = ?1 \
            AND fs_type = 'File' \
            AND thumbnail_generating = true")?;

        // Map results
        let ent_iter = stmt.query_map([source_id], |row| {
            Ok(entity::DbFsEntry {
                relative_path: row.get(0)?,
                base_path: row.get(1)?,
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
            let d = db_entry?;
            let entry = entity::FsEntry::from(d);

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list_by_missing_thumbnails(
        &self,
    ) -> Result<Vec<entity::FsEntry>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt =
            pool.prepare("SELECT \
            relative_path, base_path, source_id, fs_type, hidden, sha256, image_type, thumbnail_path, thumbnail_generating, additional_fields \
        FROM fs_entries \
        WHERE fs_type = 'File' AND thumbnail_generating = true")?;

        // Map results
        let ent_iter = stmt.query_map([], |row| {
            Ok(entity::DbFsEntry {
                relative_path: row.get(0)?,
                base_path: row.get(1)?,
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
            let d = db_entry?;
            let entry = entity::FsEntry::from(d);

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list_by_collection_id(
        &self,
        collection_id: &str,
    ) -> Result<Vec<entity::FsEntry>, crate::errors::AppError> {
        let pool = self.pool.get()?;
        let mut stmt = pool.prepare(
            "SELECT fe.* \
            FROM fs_entries fe \
            INNER JOIN collection_images ci \
                ON fe.relative_path = ci.relative_path \
                AND fe.source_id = ci.source_id \
            WHERE ci.collection_id = ?1;",
        )?;

        // Map results
        let ent_iter = stmt.query_map([collection_id], |row| {
            Ok(entity::DbFsEntry {
                relative_path: row.get(0)?,
                base_path: row.get(1)?,
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
            let d = db_entry?;
            let entry = entity::FsEntry::from(d);

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn list_by_tags(
        &self,
        includes: Vec<String>,
        excludes: Vec<String>,
    ) -> Result<Vec<entity::FsEntry>, crate::errors::AppError> {
        let pool = self.pool.get()?;

        let excludes_str = excludes
            .iter()
            .map(|x| format!("'{}'", x))
            .collect::<Vec<_>>()
            .join(", ");
        let includes_str = includes
            .iter()
            .map(|x| format!("'{}'", x))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "WITH path_results AS ( \
            SELECT fs_entries.relative_path, fs_entries.source_id, path_tags.tag_id \
            from path_tags  \
            INNER JOIN fs_entries  \
                ON fs_entries.relative_path LIKE path_tags.base_path || '%'  \
                AND fs_entries.source_id = path_tags.source_id \
            WHERE fs_entries.fs_type = 'File' \
        ), image_results AS ( \
            SELECT fs_entries.relative_path, fs_entries.source_id, image_tags.tag_id  \
            from image_tags  \
            INNER JOIN fs_entries  \
                ON fs_entries.relative_path = image_tags.relative_path  \
                AND fs_entries.source_id = image_tags.source_id \
            WHERE fs_entries.fs_type = 'File' \
        ), tags_union AS ( \
            SELECT * FROM path_results  \
            UNION  \
            SELECT * FROM image_results \
        ), excludes_results AS ( \
            SELECT * FROM tags_union \
            WHERE tag_id IN ({excludes_str})        \
        ), includes_results AS ( \
            SELECT * FROM tags_union \
            WHERE tag_id IN ({includes_str}) \
        ), results AS ( \
            SELECT inc.* FROM includes_results inc \
            LEFT JOIN excludes_results exc \
                ON inc.relative_path = exc.relative_path \
                AND inc.source_id = exc.source_id \
            WHERE exc.relative_path IS NULL \
        ) \
        SELECT fs_entries.*  \
        from results  \
        INNER JOIN fs_entries  \
            ON fs_entries.relative_path = results.relative_path  \
            AND fs_entries.source_id = results.source_id"
        );

        let mut stmt = pool.prepare(&sql)?;

        // Map results
        let ent_iter = stmt.query_map([], |row| {
            Ok(entity::DbFsEntry {
                relative_path: row.get(0)?,
                base_path: row.get(1)?,
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
            let d = db_entry?;
            let entry = entity::FsEntry::from(d);

            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn delete(
        &self,
        relative_path: &str,
        source_id: &str,
    ) -> Result<(), crate::errors::AppError> {
        let pool = self.pool.get()?;
        pool.execute(
            "DELETE FROM fs_entries WHERE \
                relative_path = ?1 AND \
                source_id = ?2",
            [relative_path, source_id],
        )?;

        Ok(())
    }

    pub fn delete_by_source_id(&self, source_id: &str) -> Result<(), crate::errors::AppError> {
        let pool = self.pool.get()?;
        pool.execute("DELETE FROM fs_entries WHERE source_id = ?1", [source_id])?;

        Ok(())
    }
}
