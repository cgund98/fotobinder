use rusqlite::Result;

pub fn init_db(pool: &crate::state::PoolType) -> Result<()> {
    pool.get().unwrap().execute(
        "CREATE TABLE IF NOT EXISTS fs_entries (
            relative_path TEXT,
            base_path TEXT,
            source_id TEXT,
            fs_type TEXT,
            hidden BOOLEAN,
            sha256 TEXT,
            image_type TEXT,
            thumbnail_path TEXT,
            thumbnail_generating BOOLEAN,
            additional_fields TEXT,

            PRIMARY KEY (relative_path, source_id),
            FOREIGN KEY(source_id) references sources(id) ON DELETE CASCADE
        )",
        (),
    )?;

    Ok(())
}
