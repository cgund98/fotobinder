use rusqlite::Result;

pub fn init_db(pool: &crate::state::PoolType) -> Result<(), crate::errors::AppError> {
    let p = pool.get()?;
    p.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id TEXT,
            parent_id TEXT NULL,
            name TEXT,

            PRIMARY KEY (id),
            FOREIGN KEY(parent_id) references tags(id) ON DELETE CASCADE
        )",
        (),
    )?;

    Ok(())
}
