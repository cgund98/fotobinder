use rusqlite::Result;

pub fn init_db(pool: &crate::state::PoolType) -> Result<()> {
    pool.get().unwrap().execute(
        "CREATE TABLE IF NOT EXISTS fs_entries (
            name TEXT,
            subpath TEXT,
            source_id TEXT,
            fs_type TEXT,
            hidden BOOLEAN,
            sha256 TEXT,
            image_type TEXT,
            thumbnail_path TEXT,
            thumbnail_generating BOOLEAN,
            additional_fields TEXT,

            PRIMARY KEY (name, subpath, source_id)
            FOREIGN KEY(source_id) references source(id)
        )",
        (),
    )?;

    Ok(())
}
