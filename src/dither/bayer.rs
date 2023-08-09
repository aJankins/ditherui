use image::{DynamicImage, Pixel};
use ndarray::{concatenate, Array, Axis, Dim};
use palette::Srgb;

use crate::{utils::numops::average, colour::utils::quantize_rgb};

use super::error::RgbImageRepr;

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

pub fn bayer_dither(image: &mut RgbImageRepr, dither_size: usize, palette: &[Srgb]) {
    let matrix = dither_matrix(dither_size);

    let ydim = image.len();
    let xdim = image.get(0).map(|row| row.len()).unwrap_or(0);

    for y in 0..ydim {
        for x in 0..xdim {
            let mut color = Srgb::from(image[y][x]).into_format::<f32>();
    
            let offset = (1.0 / 3.0)
                * (matrix
                    .get((x % dither_size, y % dither_size))
                    .unwrap_or(&0.0)
                    - 0.5) as f32;
    
            color.red = color.red + offset;
            color.blue = color.blue + offset;
            color.green = color.green + offset;
    
            image[y][x] = quantize_rgb(color, palette).into_format().into();
        }
    }
}
