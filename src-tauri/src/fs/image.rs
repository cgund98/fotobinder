use std::{cmp::min, path::Path};

use image::{
    imageops::{resize, FilterType},
    GenericImageView,
};

use crate::{data::fs_entry::entity, errors::AppError};

pub fn hash(path: &Path) -> Result<String, AppError> {
    let bytes = std::fs::read(path)?;
    let hash = sha256::digest(bytes);

    Ok(hash)
}

pub fn gen_path(entry: &entity::FsEntry, root_dir: &str) -> std::path::PathBuf {
    Path::new(root_dir).join(&entry.subpath).join(&entry.name)
}

pub fn gen_thumbnail(
    entry: &mut entity::FsEntry,
    root_dir: &str,
    thumbnail_dir: &Path,
) -> Result<(), AppError> {
    if entry.fs_type == entity::FileType::Directory {
        return Err(AppError::BadRequest(String::from(
            "Attempting to generate thumbnail for a directory.",
        )));
    }

    let path = gen_path(entry, root_dir);

    let img = image::open(path.clone())?;

    // Find new scale
    let (width, height) = img.dimensions();
    let min_dimension = 250;
    let scale = min(width, height) / min_dimension;

    // Resize image
    let thumb = resize(&img, width / scale, height / scale, FilterType::Gaussian);

    // Create output file
    let mut fname = sha256::digest(path.to_string_lossy().as_bytes());
    fname.push_str(".jpg");
    let thumbnail_path = thumbnail_dir.join(fname.clone());
    let mut output = std::fs::File::create(thumbnail_path.clone())?;
    thumb.write_to(&mut output, image::ImageFormat::Jpeg)?;

    entry.thumbnail_path = fname;

    Ok(())
}
