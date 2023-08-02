use ::image::ImageError;

pub mod image;
pub mod numops;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ImageError(ImageError),
    Base64DecodeError(base64::DecodeError),
    ReqwestError(reqwest::Error),
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

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ReqwestError(value)
    }
}

pub type ImageFilterResult<T> = Result<T, Error>;
