use image::DynamicImage;
use palette::Srgb;

use crate::colour::utils::{ONE_BIT, grayscale_rgb, quantize_rgb, compute_rgb_error};

pub fn basic_dither(image: DynamicImage, is_mono: bool, palette: &[Srgb]) -> DynamicImage {
    let mut error = (0.0, 0.0, 0.0);
    let mut image = image.into_rgb8();

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        let mut color = Srgb::from(pixel.0).into_format::<f32>();
        color.red = color.red + error.0;
        color.blue = color.blue + error.1;
        color.green = color.green + error.2;
        if is_mono {
            color = grayscale_rgb(color);
        }
        let quantized = quantize_rgb(color, palette);

        error = compute_rgb_error(color, quantized);

        pixel.0 = quantized.into_format().into();
    }

    DynamicImage::ImageRgb8(image)
}

pub fn basic_mono_dither(image: DynamicImage) -> DynamicImage {
    basic_dither(image, true, ONE_BIT)
}

pub fn basic_colour_dither(image: DynamicImage, palette: &[Srgb]) -> DynamicImage {
    basic_dither(image, false, palette)
}
