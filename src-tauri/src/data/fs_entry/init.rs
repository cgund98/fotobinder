use rusqlite::Result;

pub fn init_db(pool: &crate::state::PoolType) -> Result<()> {
    pool.get().unwrap().execute(
        "CREATE TABLE IF NOT EXISTS fs_entries (
            name TEXT PRIMARY KEY,
            subpath TEXT PRIMARY KEY,
            source TEXT PRIMARY KEY,
            fs_type TEXT,
            name TEXT,
            hidden BOOLEAN,
            sha256 TEXT,
            image_type TEXT,
            thumbnail_path TEXT,
            additional_fields TEXT,

            FOREIGN KEY(source) references source(id)
        )",
        (),
    )?;

    Ok(())
}
