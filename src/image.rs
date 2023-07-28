use image::{self, DynamicImage};
use image::io::Reader as ImageReader;

pub fn load_image(path: &str) -> DynamicImage {
    let opened = ImageReader::open(path).expect(format!("Couldn't open image at '{}'", path).as_str());
    let decoded = opened.decode().expect("Failed to decode image.");
    return decoded;
}