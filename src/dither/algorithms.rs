use image::{DynamicImage, Pixel};

use crate::utils::u8ops::average;

pub enum Algorithms {
    BasicMono
}

impl Algorithms {
    pub fn dither(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::BasicMono => basic_dither(image)
        }
    }
}

fn basic_dither(image: DynamicImage) -> DynamicImage {
    let collapser = |num: i16| if num < 128 { 0 } else { 255 };

    let mut error = 0;
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let mono = average(pixel.channels()) as i16 + error;
        let threshold = collapser(mono);
        error = mono - threshold;
        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}