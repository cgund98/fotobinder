use rusqlite::Result;

pub fn init_db(pool: &crate::state::PoolType) -> Result<()> {
    pool.get().unwrap().execute(
        "CREATE TABLE IF NOT EXISTS path_tags (
            base_path TEXT,
            source_id TEXT,
            tag_id TEXT,

            PRIMARY KEY (base_path, source_id, tag_id),
            FOREIGN KEY (source_id) references sources(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) references tags(id) ON DELETE CASCADE
        )",
        (),
    )?;

    Ok(())
}
