use image::{DynamicImage, Pixel};
use ndarray::{concatenate, Array, Axis, Dim};

use crate::{pixel::rgb::RgbPixel, utils::numops::average};

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

pub fn bayer_mono_dither(image: DynamicImage, dither_size: usize) -> DynamicImage {
    // let mut rgb8_image = image.into_rgb8();
    let matrix = dither_matrix(dither_size);
    let mut rgb8_image = image.into_rgb8();

    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let mono = average(pixel.channels()) as u8;
        let mapped_pixel = mono as f64
            + (255.0
                * (matrix
                    .get((xs % dither_size, ys % dither_size))
                    .unwrap_or(&0.0)
                    - 0.5));

        let threshold = if mapped_pixel > 128.0 { 255 } else { 0 };

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

pub fn bayer_dither(image: DynamicImage, dither_size: usize, palette: &[RgbPixel]) -> DynamicImage {
    // let mut rgb8_image = image.into_rgb8();
    let matrix = dither_matrix(dither_size);
    let mut rgb8_image = image.into_rgb8();

    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let rgb = RgbPixel::from(&*pixel);

        let offset = (1.0 / 3.0)
            * (matrix
                .get((xs % dither_size, ys % dither_size))
                .unwrap_or(&0.0)
                - 0.5) as f32;

        (pixel[0], pixel[1], pixel[2]) = rgb
            .add_error((offset, offset, offset))
            .quantize(palette)
            .get_u8();
    }

    DynamicImage::ImageRgb8(rgb8_image)
}
