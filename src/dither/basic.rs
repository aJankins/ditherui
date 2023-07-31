use image::DynamicImage;

use crate::colour::pixel::{MonoPixel, TWO_BIT, RgbPixel};

pub fn basic_mono_dither(image: DynamicImage) -> DynamicImage {
    let mut error = 0;
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let mono = MonoPixel::from(&*pixel).add_error(error);
        let quantized = mono.quantize(TWO_BIT);

        error = mono.get_error(&quantized);

        let quantized_val = quantized.get();

        pixel[0] = quantized_val;
        pixel[1] = quantized_val;
        pixel[2] = quantized_val;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

pub fn basic_colour_dither(image: DynamicImage, palette: &[RgbPixel]) -> DynamicImage {
    let mut error = (0 as i32, 0 as i32, 0 as i32);
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let rgb = RgbPixel::from(&*pixel).add_error((
            error.0,
            error.1,
            error.2,
        ));

        let quantized = rgb.quantize(palette);

        error = rgb.get_error(&quantized);

        (pixel[0], pixel[1], pixel[2]) = quantized.get()
    }

    DynamicImage::ImageRgb8(rgb8_image)
}