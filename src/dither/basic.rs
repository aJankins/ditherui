use image::DynamicImage;
use palette::Srgb;

use crate::colour::utils::{grayscale_rgb, quantize_rgb, compute_rgb_error};

pub fn basic_dither(image: DynamicImage, palette: &[Srgb]) -> DynamicImage {
    let mut error = (0.0, 0.0, 0.0);
    let mut image = image.into_rgb8();

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        let mut color = Srgb::from(pixel.0).into_format::<f32>();
        color.red = color.red + error.0;
        color.blue = color.blue + error.1;
        color.green = color.green + error.2;
        if false {
            color = grayscale_rgb(color);
        }
        let quantized = quantize_rgb(color, palette);

        error = compute_rgb_error(color, quantized);

        pixel.0 = quantized.into_format().into();
    }

    DynamicImage::ImageRgb8(image)
}
