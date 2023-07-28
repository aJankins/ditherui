use image::{DynamicImage, Pixel};

use crate::utils::u8ops::average;

pub fn burkes_mono_dither(image: DynamicImage) -> DynamicImage {
    let collapser = |num: i64| if num < 128 { 0 } else { 255 };

    let mut rgb8_image = image.into_rgb8();
    let (xdim, ydim) = rgb8_image.dimensions();
    let mut error_matrix = vec![vec![0 as i64; (xdim+2) as usize]; (ydim+2) as usize];


    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let mono = average(pixel.channels()) as i64 + *(&error_matrix[ys][xs]);
        let threshold = collapser(mono);

        let error = mono - threshold;

        let (eight32, four32, two32) = (
            ((error * 8) / 32),
            ((error * 4) / 32),
            ((error * 2) / 32)
        );

        // 1st row
        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + eight32;
        error_matrix[ys][xs+2] = &error_matrix[ys][xs+2] + four32;
        // 2nd row
        if xs > 1 {
            error_matrix[ys+1][xs-2] = error_matrix[ys+1][xs-2] + two32;
        }
        if xs > 0 {
            error_matrix[ys+1][xs-1] = error_matrix[ys+1][xs-1] + four32;
        }
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + eight32;
        error_matrix[ys+1][xs+1] = &error_matrix[ys+1][xs+1] + four32;
        error_matrix[ys+1][xs+2] = &error_matrix[ys+1][xs+2] + two32;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}