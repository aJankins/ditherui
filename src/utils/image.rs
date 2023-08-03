use std::io::Cursor;

use base64::Engine;
use image::io::Reader as ImageReader;
use image::{self, imageops, DynamicImage, GenericImageView};

use super::ImageFilterResult;

pub fn load_image(path: &str) -> ImageFilterResult<DynamicImage> {
    Ok(ImageReader::open(path)?.decode()?)
}

pub fn image_to_b64(image: &DynamicImage) -> ImageFilterResult<String> {
    let mut buf = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
        .unwrap();
    Ok(base64::engine::general_purpose::STANDARD_NO_PAD.encode(buf))
}

pub fn b64_to_image(b64: &str) -> ImageFilterResult<DynamicImage> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .unwrap();
    Ok(image::load_from_memory(&bytes)?)
}

pub fn resize_image(image: &DynamicImage, factor: f32) -> DynamicImage {
    let (x, y) = image.dimensions();
    let mul = |int: u32, float: f32| (int as f32 * float) as u32;
    image.resize(mul(x, factor), mul(y, factor), imageops::Gaussian)
}

pub fn resize_image_with_max_dim(image: &DynamicImage, maxdim: u32) -> DynamicImage {
    let (x, y) = image.dimensions();
    if maxdim < x.max(y) {
        resize_image(&image, maxdim as f32 / x.max(y) as f32)
    } else {
        image.clone()
    }
}

pub fn load_image_with_max_dim(path: &str, maxdim: u32) -> ImageFilterResult<DynamicImage> {
    let image = load_image(path)?;
    Ok(resize_image_with_max_dim(&image, maxdim))
}

pub fn load_image_from_url(url: &str) -> ImageFilterResult<DynamicImage> {
    let img_bytes = reqwest::blocking::get(url)?.bytes()?;
    Ok(image::load_from_memory(&img_bytes)?)
}

pub fn load_image_from_url_with_max_dim(url: &str, maxdim: u32) -> ImageFilterResult<DynamicImage> {
    let image = load_image_from_url(url)?;
    Ok(resize_image_with_max_dim(&image, maxdim))
}
