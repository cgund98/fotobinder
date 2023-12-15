use crate::errors::AppError;

pub fn init_db(pool: &crate::state::PoolType) -> Result<(), AppError> {
    let p = pool.get()?;
    p.execute(
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
