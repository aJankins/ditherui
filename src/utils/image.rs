use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};
use std::slice::SliceIndex;

use base64::Engine;
use image::codecs::gif::GifDecoder;
use image::io::Reader as ImageReader;
use image::{self, imageops, DynamicImage, GenericImageView, Frame, AnimationDecoder};

type UtilResult<T> = Result<T,Box<dyn Error>>;

pub type RgbPixelRepr = [u8; 3];
pub type RgbaPixelRepr = [u8; 4];

pub type RgbImageRepr = Vec<Vec<RgbPixelRepr>>;
pub type RgbaImageRepr = Vec<Vec<RgbaPixelRepr>>;

pub(crate) fn get_dimensions_of_matrix<T>(
    matrix: &Vec<Vec<T>>
) -> (usize, usize)
{
    let ydim = matrix.len();
    let xdim = matrix.get(0).map(|row| row.len()).unwrap_or(0);
    (xdim, ydim)
}