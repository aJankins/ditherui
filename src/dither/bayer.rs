use image::{DynamicImage, Pixel};
use ndarray::{concatenate, Array, Axis, Dim};
use palette::Srgb;

use crate::{utils::numops::average, colour::utils::quantize_rgb};

fn dither_matrix(n: usize) -> Array<f64, Dim<[usize; 2]>> {
    if n == 1 {
        return Array::<f64, _>::zeros((1, 1));
    }

    let nested_matrix = dither_matrix(n / 2);
    let multiplier = n.pow(2) as f64;

    let first = multiplier * nested_matrix.clone();
    let second = multiplier * nested_matrix.clone() + 2.;
    let third = multiplier * nested_matrix.clone() + 3.;
    let fourth = multiplier * nested_matrix.clone() + 1.;

    let first_col = concatenate(Axis(0), &[first.view(), third.view()]).unwrap();
    let second_col = concatenate(Axis(0), &[second.view(), fourth.view()]).unwrap();

    (1. / multiplier) * concatenate(Axis(1), &[first_col.view(), second_col.view()]).unwrap()
}

pub fn bayer_dither(image: DynamicImage, dither_size: usize, palette: &[Srgb]) -> DynamicImage {
    // let mut rgb8_image = image.into_rgb8();
    let matrix = dither_matrix(dither_size);
    let mut rgb8_image = image.into_rgb8();

    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let mut color = Srgb::from(pixel.0).into_format::<f32>();

        let offset = (1.0 / 3.0)
            * (matrix
                .get((xs % dither_size, ys % dither_size))
                .unwrap_or(&0.0)
                - 0.5) as f32;

        color.red = color.red + offset;
        color.blue = color.blue + offset;
        color.green = color.green + offset;

        pixel.0 = quantize_rgb(color, palette).into_format().into();
    }

    DynamicImage::ImageRgb8(rgb8_image)
}
