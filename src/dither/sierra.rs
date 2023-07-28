use image::{DynamicImage, Pixel};

use crate::utils::u8ops::average;

pub fn sierra_mono_dither(image: DynamicImage) -> DynamicImage {
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

        let (five32, four32, three32, two32) = (
            ((error * 5) / 32),
            ((error * 4) / 32),
            ((error * 3) / 32),
            ((error * 2) / 32)
        );

        // 1st row
        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + five32;
        error_matrix[ys][xs+2] = &error_matrix[ys][xs+2] + three32;
        // 1st col
        if xs > 1 {
            error_matrix[ys+1][xs-2] = error_matrix[ys+1][xs-2] + two32;
        }
        // 2nd col
        if xs > 0 {
            error_matrix[ys+1][xs-1] = error_matrix[ys+1][xs-1] + four32;
            error_matrix[ys+2][xs-1] = error_matrix[ys+2][xs-1] + two32;
        }
        // other cols
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + five32;
        error_matrix[ys+2][xs] = &error_matrix[ys+2][xs] + three32;

        error_matrix[ys+1][xs+1] = &error_matrix[ys+1][xs+1] + four32;
        error_matrix[ys+2][xs+1] = &error_matrix[ys+2][xs+1] + two32;

        error_matrix[ys+1][xs+2] = &error_matrix[ys+1][xs+2] + two32;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

pub fn two_row_sierra_mono_dither(image: DynamicImage) -> DynamicImage {
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

        let (four16, three16, two16, one16) = (
            ((error * 4) / 16),
            ((error * 3) / 16),
            ((error * 2) / 16),
            ((error * 1) / 16)
        );

        // 1st row
        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + four16;
        error_matrix[ys][xs+2] = &error_matrix[ys][xs+2] + three16;
        // 2nd row
        if xs > 1 {
            error_matrix[ys+1][xs-2] = error_matrix[ys+1][xs-2] + one16;
        }
        if xs > 0 {
            error_matrix[ys+1][xs-1] = error_matrix[ys+1][xs-1] + two16;
        }
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + three16;
        error_matrix[ys+1][xs+1] = &error_matrix[ys+1][xs+1] + two16;
        error_matrix[ys+1][xs+2] = &error_matrix[ys+1][xs+2] + one16;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

pub fn sierra_lite_mono_dither(image: DynamicImage) -> DynamicImage {
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
        
        let (two4, one4) = (
            ((error * 2) / 4),
            ((error * 1) / 4),
        );

        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + two4;
        if xs > 0 {
            error_matrix[ys+1][xs-1] = error_matrix[ys+1][xs-1] + one4;
        }
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + one4;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}