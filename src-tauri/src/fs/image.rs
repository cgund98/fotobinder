use std::path::Path;

use crate::errors::AppError;

pub fn hash(path: &Path) -> Result<String, AppError> {
    let bytes = std::fs::read(path)?;
    let hash = sha256::digest(bytes);

    Ok(hash)
}
