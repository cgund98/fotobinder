pub fn init_db(pool: &crate::state::PoolType) -> Result<(), crate::errors::AppError> {
    let p = pool.get()?;
    p.execute(
        "CREATE TABLE IF NOT EXISTS collection_images (
            relative_path TEXT,
            source_id TEXT,
            collection_id TEXT,

            PRIMARY KEY (relative_path, source_id, collection_id),
            FOREIGN KEY (source_id) references sources(id) ON DELETE CASCADE,
            FOREIGN KEY (collection_id) references collections(id) ON DELETE CASCADE
        )",
        (),
    )?;

    Ok(())
}
