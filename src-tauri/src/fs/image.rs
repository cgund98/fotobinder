use std::{cmp::min, path::Path};

use fast_image_resize as fr;
use image::GenericImageView;
use std::io::BufWriter;
use std::num::NonZeroU32;

use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};

use crate::{data::fs_entry::entity, errors::AppError};

pub fn hash(path: &Path) -> Result<String, AppError> {
    let bytes = std::fs::read(path)?;
    let hash = sha256::digest(bytes);

    Ok(hash)
}

// Generate the path of an original entry
pub fn gen_origin_path(entry: &entity::FsEntry, root_dir: &str) -> std::path::PathBuf {
    Path::new(root_dir).join(&entry.relative_path)
}

// Generate the path of a thumbnail image
pub fn gen_thumbnail_path(origin_path: &Path, thumbnails_path: &Path) -> std::path::PathBuf {
    let mut fname = sha256::digest(origin_path.to_string_lossy().as_bytes());
    fname.push_str(".jpg");

    thumbnails_path.join(fname.clone())
}

// Generate a small thumbnail for viewing in the application UI
pub fn gen_thumbnail(source: String, destination: String) -> Result<(u32, u32), AppError> {
    let source_path = Path::new(&source);
    let dest_path = Path::new(&destination);

    let reader = ImageReader::open(source_path)?;
    let img = reader.decode()?;
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();
    let mut src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )?;

    // Multiple RGB channels of source image by alpha channel
    // (not required for the Nearest algorithm)
    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;

    // Create container for data of destination image
    let (w, h) = img.dimensions();
    let scale = min(w, h) / 250;
    let dst_width = NonZeroU32::new(w / scale).unwrap();
    let dst_height = NonZeroU32::new(h / scale).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Bilinear));
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    // Divide RGB channels of destination image by alpha
    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    // Create output file
    let output = std::fs::File::create(dest_path)?;

    // Write destination image as PNG-file
    let mut result_buf = BufWriter::new(output);
    JpegEncoder::new(&mut result_buf).write_image(
        dst_image.buffer(),
        dst_width.get(),
        dst_height.get(),
        ColorType::Rgba8,
    )?;

    Ok((w, h))
}
