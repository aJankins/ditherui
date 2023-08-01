use ::image::ImageError;

pub mod numops;
pub mod image;

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

pub type ImageFilterResult<T> = Result<T, Error>;