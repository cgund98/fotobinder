use rusqlite::Result;

pub fn init_db(pool: &crate::state::PoolType) -> Result<(), crate::errors::AppError> {
    let p = pool.get()?;
    p.execute(
        "CREATE TABLE IF NOT EXISTS sources (
            id TEXT PRIMARY KEY,
            source_type TEXT,
            name TEXT,
            path TEXT,
            synced_at DATETIME
        )",
        (),
    )?;

    Ok(())
}
