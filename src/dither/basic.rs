use image::{DynamicImage, Pixel};

use crate::utils::u8ops::average;

use super::{truple_from_rgb, get_closest_color_to};

pub fn basic_mono_dither(image: DynamicImage) -> DynamicImage {
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

pub fn basic_colour_dither(image: DynamicImage, palette: &[(u8, u8, u8)]) -> DynamicImage {
    let mut error = (0 as i16, 0 as i16, 0 as i16);
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let (r, g, b) = truple_from_rgb(pixel);
        let raw_pixel = (
            ((r as i16)+0*error.0) as u8,
            ((g as i16)+0*error.1) as u8,
            ((b as i16)+0*error.2) as u8
        );

        let set_to = get_closest_color_to(raw_pixel, palette);

        error.0 = raw_pixel.0 as i16 - set_to.0 as i16;
        error.1 = raw_pixel.1 as i16 - set_to.1 as i16;
        error.2 = raw_pixel.2 as i16 - set_to.2 as i16;

        pixel[0] = set_to.0;
        pixel[1] = set_to.1;
        pixel[2] = set_to.2;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}