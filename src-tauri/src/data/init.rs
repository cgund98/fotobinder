use rusqlite::Result;

use crate::data::collection;
use crate::data::collection_image;
use crate::data::fs_entry;
use crate::data::image_tag;
use crate::data::path_tag;
use crate::data::source;
use crate::data::tag;

pub fn init_db(pool: &crate::state::PoolType) -> Result<()> {
    // Enable foreign keys
    pool.get()
        .unwrap()
        .execute("PRAGMA foreign_keys = ON", [])?;

    // Initialize entity tables
    source::init::init_db(pool)?;
    fs_entry::init::init_db(pool)?;
    tag::init::init_db(pool)?;
    image_tag::init::init_db(pool)?;
    path_tag::init::init_db(pool)?;
    collection::init::init_db(pool)?;
    collection_image::init::init_db(pool)?;

    Ok(())
}
