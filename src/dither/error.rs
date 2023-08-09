use image::{ImageBuffer, Rgb};
use palette::Srgb;

use crate::{
    utils::numops::map_to_2d,
    colour::utils::{quantize_rgb, ONE_BIT, compute_rgb_error, grayscale_rgb},
};

pub fn error_propagate_through_pixels(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    matrix: &[(i8, i8, u8)],
    total_portions: u16,
) {
    let (xdim, ydim) = image.dimensions();

    for i in xdim as u64..(xdim * ydim) as u64 {
        let (x, y) = map_to_2d(i, xdim);

        let error = {
            let pixel = image.get_pixel_mut(x, y);
            let srgb = Srgb::from(pixel.0).into_format::<f32>();
            let srgb = grayscale_rgb(srgb);
            let quantized = quantize_rgb(srgb, ONE_BIT);
            pixel.0 = quantized.into_format().into();
            compute_rgb_error(srgb, quantized)
        };

        let error = (
            (error.0 * 255.0) as i32,
            (error.1 * 255.0) as i32,
            (error.2 * 255.0) as i32,
        );

        for (x_off, y_off, portion) in matrix.iter() {
            let (x_err, y_err) = (
                (x as i64 + *x_off as i64) as u32,
                (y as i64 + *y_off as i64) as u32,
            );
            let pixel = if x_err < xdim && y_err < ydim {
                image.get_pixel_mut_checked(x_err, y_err)
            } else {
                None
            };

            if let Some(pixel) = pixel {
                pixel.0 = [
                    (pixel[0] as i32 + (error.0 * *portion as i32) / total_portions as i32).clamp(0, 255) as u8,
                    (pixel[1] as i32 + (error.1 * *portion as i32) / total_portions as i32).clamp(0, 255) as u8,
                    (pixel[2] as i32 + (error.2 * *portion as i32) / total_portions as i32).clamp(0, 255) as u8,
                ];
            }
        }
    }
}

pub fn error_propagate_through_pixels_rgb(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    matrix: &[(i8, i8, u8)],
    total_portions: u16,
    palette: &[Srgb],
) {
    let (xdim, ydim) = image.dimensions();

    for i in xdim as u64..(xdim * ydim) as u64 {
        let (x, y) = map_to_2d(i, xdim);
        let error = {
            let pixel = image.get_pixel_mut(x, y);
            let rgb = Srgb::from(pixel.0).into_format::<f32>();
            let quantized = quantize_rgb(rgb, palette);
            pixel.0 = quantized.into_format().into();
            compute_rgb_error(rgb, quantized)
        };

        let error = (
            (error.0 * 255.0) as i32,
            (error.1 * 255.0) as i32,
            (error.2 * 255.0) as i32,
        );

        for (x_off, y_off, portion) in matrix.iter() {
            let (x_err, y_err) = (
                (x as i64 + *x_off as i64) as u32,
                (y as i64 + *y_off as i64) as u32,
            );
            let pixel = if x_err < xdim && y_err < ydim {
                image.get_pixel_mut_checked(x_err, y_err)
            } else {
                None
            };

            if let Some(pixel) = pixel {
                pixel[0] = (pixel[0] as i32 + (error.0 * *portion as i32) / total_portions as i32)
                    .clamp(0, 255) as u8;
                pixel[1] = (pixel[1] as i32 + (error.1 * *portion as i32) / total_portions as i32)
                    .clamp(0, 255) as u8;
                pixel[2] = (pixel[2] as i32 + (error.2 * *portion as i32) / total_portions as i32)
                    .clamp(0, 255) as u8;
            }
        }
    }
}

macro_rules! error_prop_rgb_fn {
    ($fn_name:ident, $matrix:expr, $portion_amnt:expr) => {
        pub fn $fn_name(image: DynamicImage, palette: &[Srgb]) -> DynamicImage {
            let mut rgb8_image = image.into_rgb8();
            error_propagate_through_pixels_rgb(&mut rgb8_image, $matrix, $portion_amnt, palette);
            DynamicImage::ImageRgb8(rgb8_image)
        }
    };
}


macro_rules! error_prop_mod {
    ($mod_name:ident, [$($matrix:tt)*] / $portion_amnt:expr) => {
        pub mod $mod_name {
            use image::DynamicImage;
            use palette::Srgb;
            use crate::{
                dither::{
                    error::{
                        error_propagate_through_pixels_rgb
                    },
                },
            };

            static PROPAGATION_MATRIX: &[(i8, i8, u8)] = &[$($matrix)*];

            error_prop_rgb_fn!(dither_rgb, PROPAGATION_MATRIX, $portion_amnt);
        }
    };
}

error_prop_mod!(
    floyd_steinberg,
        [
                                 (1, 0, 7),
            (-1, 1, 5),(0, 1, 3),(1, 1, 1),
        ] / 16
);

error_prop_mod!(
    jarvis_judice_ninke,
        [
                                            (1, 0, 7),(2, 0, 5),
            (-2, 1, 3),(-1, 1, 5),(0, 1, 7),(1, 1, 5),(2, 1, 3),
            (-2, 2, 1),(-1, 2, 3),(0, 2, 5),(1, 2, 3),(2, 2, 1),
        ] / 48
);

error_prop_mod!(
    atkinson,
        [
                                 (1, 0, 1),(2, 0, 1),
            (-1, 1, 1),(0, 1, 1),(1, 1, 1),
                       (0, 2, 1)
        ] / 8
);

error_prop_mod!(
    burkes,
        [
                                               (1, 0, 8), (2, 0, 4),
            (-2, 1, 2), (-1, 1, 4), (0, 1, 8), (1, 1, 4), (2, 1, 2),
        ] / 32
);

error_prop_mod!(
    stucki,
        [
                                            (1, 0, 8),(2, 0, 4),
            (-2, 1, 2),(-1, 1, 4),(0, 1, 8),(1, 1, 4),(2, 1, 2),
            (-2, 2, 1),(-1, 2, 2),(0, 2, 4),(1, 2, 2),(2, 2, 1),
        ] / 42
);

pub mod sierra {
    use image::DynamicImage;
    use palette::Srgb;
    use crate::dither::error::{
        error_propagate_through_pixels_rgb
    };

    static PROPAGATION_MATRIX: &[(i8, i8, u8)] =         &[
                                        (1, 0, 5),(2, 0, 3),
        (-2, 1, 2),(-1, 1, 4),(0, 1, 5),(1, 1, 4),(2, 1, 2),
                   (-1, 2, 2),(0, 2, 3),(1, 2, 2),
    ];

    error_prop_rgb_fn!(dither_rgb, PROPAGATION_MATRIX, 32);

    error_prop_mod!(
        two_row,
            [
                                                (1, 0, 4),(2, 0, 3),
                (-2, 1, 1),(-1, 1, 2),(0, 1, 3),(1, 1, 2),(2, 1, 1),
            ] / 16
    );

    error_prop_mod!(
        lite,
            [
                                     (1, 0, 2),
                (-1, 1, 1),(0, 1, 1)
            ] / 4
    );
}