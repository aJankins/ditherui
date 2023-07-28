use image::{DynamicImage, Pixel};

use crate::utils::u8ops::average;

pub fn jarvis_judice_ninke_mono_dither(image: DynamicImage) -> DynamicImage {
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

        let (seven48, five48, three48, one48) = (
            ((error * 7) / 48),
            ((error * 5) / 48),
            ((error * 3) / 48),
            ((error * 1) / 48)
        );

        // 1st row
        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + seven48;
        error_matrix[ys][xs+2] = &error_matrix[ys][xs+2] + five48;
        // 1st col
        if xs > 1 {
            error_matrix[ys+1][xs-2] = error_matrix[ys+1][xs-2] + three48;
            error_matrix[ys+2][xs-2] = error_matrix[ys+2][xs-2] + one48;
        }
        // 2nd col
        if xs > 0 {
            error_matrix[ys+1][xs-1] = error_matrix[ys+1][xs-1] + five48;
            error_matrix[ys+2][xs-1] = error_matrix[ys+2][xs-1] + three48;
        }
        // other cols
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + seven48;
        error_matrix[ys+2][xs] = &error_matrix[ys+2][xs] + five48;

        error_matrix[ys+1][xs+1] = &error_matrix[ys+1][xs+1] + five48;
        error_matrix[ys+2][xs+1] = &error_matrix[ys+2][xs+1] + three48;

        error_matrix[ys+1][xs+2] = &error_matrix[ys+1][xs+2] + three48;
        error_matrix[ys+2][xs+2] = &error_matrix[ys+2][xs+2] + one48;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}