use rusqlite::Result;

pub fn init_db(pool: &crate::state::PoolType) -> Result<(), crate::errors::AppError> {
    let p = pool.get()?;
    p.execute(
        "CREATE TABLE IF NOT EXISTS image_tags (
            relative_path TEXT,
            source_id TEXT,
            tag_id TEXT,

            PRIMARY KEY (relative_path, source_id, tag_id),
            FOREIGN KEY (source_id) references sources(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) references tags(id) ON DELETE CASCADE
        )",
        (),
    )?;

    Ok(())
}
