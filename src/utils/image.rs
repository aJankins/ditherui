use std::io::Cursor;

use base64::Engine;
use image::io::Reader as ImageReader;
use image::{self, DynamicImage};

use super::ImageFilterResult;

pub fn load_image(path: &str) -> ImageFilterResult<DynamicImage> {
    Ok(ImageReader::open(path)?.decode()?)
}

pub fn image_to_b64(image: DynamicImage) -> ImageFilterResult<String> {
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
