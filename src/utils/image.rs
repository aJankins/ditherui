use std::io::Cursor;

use base64::Engine;
use image::{self, DynamicImage, ImageError};
use image::io::Reader as ImageReader;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ImageError(ImageError),
    Base64DecodeError(base64::DecodeError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Error::ImageError(value)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(value: base64::DecodeError) -> Self {
        Error::Base64DecodeError(value)
    }
}

pub fn load_image(path: &str) -> Result<DynamicImage, Error> {
    Ok(ImageReader::open(path)?.decode()?)
}

pub fn image_to_b64(image: DynamicImage) -> Result<String, Error> {
    let mut buf = Vec::new();
    image.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).unwrap();
    Ok(base64::engine::general_purpose::STANDARD_NO_PAD.encode(buf))
}

pub fn b64_to_image(b64: &str) -> Result<DynamicImage, Error> {
    let bytes = base64::engine::general_purpose::STANDARD_NO_PAD.decode(b64).unwrap();
    Ok(image::load_from_memory(&bytes)?)
}