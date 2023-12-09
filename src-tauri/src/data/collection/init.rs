use rusqlite::Result;

pub fn init_db(pool: &crate::state::PoolType) -> Result<()> {
    pool.get().unwrap().execute(
        "CREATE TABLE IF NOT EXISTS collections (
            id TEXT,
            parent_id TEXT NULL,
            name TEXT,

            PRIMARY KEY (id),
            FOREIGN KEY(parent_id) references collections(id) ON DELETE CASCADE
        )",
        (),
    )?;

    Ok(())
}
