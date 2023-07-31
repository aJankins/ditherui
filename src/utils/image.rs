use std::io::Cursor;

use base64::Engine;
use image::{self, DynamicImage};
use image::io::Reader as ImageReader;

pub fn load_image(path: &str) -> DynamicImage {
    let opened = ImageReader::open(path).expect(format!("Couldn't open image at '{}'", path).as_str());
    let decoded = opened.decode().expect("Failed to decode image.");
    return decoded;
}

pub fn image_to_b64(image: DynamicImage) -> String {
    let mut buf = Vec::new();
    image.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).unwrap();
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(buf)
}