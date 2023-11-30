use rusqlite::Result;

use crate::data::fs_entry;
use crate::data::source;

pub fn init_db(pool: &crate::state::PoolType) -> Result<()> {
    source::init::init_db(pool)?;
    fs_entry::init::init_db(pool)?;

    Ok(())
}
