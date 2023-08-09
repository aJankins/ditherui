use image::DynamicImage;
use palette::Srgb;

use crate::colour::utils::{grayscale_rgb, quantize_rgb, compute_rgb_error};

use super::error::RgbImageRepr;

pub fn basic_dither(image: &mut RgbImageRepr, palette: &[Srgb]) {
    let mut error = (0.0, 0.0, 0.0);

    let ydim = image.len();
    let xdim = image.get(0).map(|row| row.len()).unwrap_or(0);

    for y in 0..ydim {
        for x in 0..xdim {
            let mut color = Srgb::from(image[y][x]).into_format::<f32>();
            color.red = color.red + error.0;
            color.blue = color.blue + error.1;
            color.green = color.green + error.2;
            if false {
                color = grayscale_rgb(color);
            }
            let quantized = quantize_rgb(color, palette);
    
            error = compute_rgb_error(color, quantized);
    
            image[y][x] = quantized.into_format().into();
        }
    }
}
