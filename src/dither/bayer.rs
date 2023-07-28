use std::ops::Mul;

use image::{DynamicImage, Pixel};
use ndarray::{Array, Dim, concatenate, Axis};

use crate::utils::u8ops::average;

const DITHER_MATRIX_2x2: [f64; 4] = [-0.375, 0.125, 0.375, -0.125];

fn dither_matrix(n: usize) -> Array<f64, Dim<[usize; 2]>> {
    if n == 1 { return Array::<f64, _>::zeros((1, 1)) }

    let nested_matrix = dither_matrix(n/2);
    let multiplier = n.pow(2) as f64;

    let first = multiplier * nested_matrix.clone();
    let second = multiplier * nested_matrix.clone() + 2.;
    let third = multiplier * nested_matrix.clone() + 3.;
    let fourth = multiplier * nested_matrix.clone() + 1.;

    let first_col = concatenate(Axis(0), &[first.view(), third.view()]).unwrap();
    let second_col = concatenate(Axis(0), &[second.view(), fourth.view()]).unwrap();

    (1. / multiplier) * concatenate(Axis(1), &[first_col.view(), second_col.view()]).unwrap()
}

pub fn bayer_mono_dither(image: DynamicImage, dither_size: usize) -> DynamicImage {
    // let mut rgb8_image = image.into_rgb8();
    let matrix = dither_matrix(dither_size) * 255.;
    let mut rgb8_image = image.into_rgb8();

    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let i = xs % dither_size;
        let j = ys % dither_size;

        let dither = DITHER_MATRIX_2x2[(xs & 1) + (ys & 1) * 2];

        let mono = average(pixel.channels()) as u8;
        let threshold = if mono > (*matrix.get((i, j)).unwrap_or(&0.0) as u8) { 255 } else { 0 };

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}