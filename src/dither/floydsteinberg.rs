use image::{DynamicImage, Pixel};

use crate::utils::u8ops::average;

use super::pixel::{MonoPixel, TWO_BIT};

pub fn floyd_steinberg_mono_dither(image: DynamicImage) -> DynamicImage {
    let mut rgb8_image = image.into_rgb8();
    let (xdim, ydim) = rgb8_image.dimensions();
    let mut error_matrix = vec![vec![0 as i32; (xdim+1) as usize]; (ydim+1) as usize];


    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let mono = MonoPixel::mono_from(pixel).add_error(error_matrix[ys][xs]);
        let quantized = mono.quantize(TWO_BIT);

        let error = mono.get_error(&quantized);

        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + ((error * 7) / 16) as i32;
        if xs > 0 { error_matrix[ys+1][xs-1] = &error_matrix[ys+1][xs-1] + ((error * 5) / 16) as i32; }
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + ((error * 3) / 16) as i32;
        error_matrix[ys+1][xs+1] = &error_matrix[ys+1][xs+1] + ((error * 1) / 16) as i32;

        pixel[0] = quantized.get();
        pixel[1] = quantized.get();
        pixel[2] = quantized.get();
    }

    DynamicImage::ImageRgb8(rgb8_image)
}