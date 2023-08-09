use image::{ImageBuffer, Rgb};
use palette::Srgb;

use crate::{
    utils::numops::map_to_2d,
    colour::utils::{quantize_rgb, ONE_BIT, compute_rgb_error, grayscale_rgb},
};

pub type RgbImageRepr = Vec<Vec<[u8; 3]>>;

pub fn error_propagate_through_pixels_rgb(
    // image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    image: &mut RgbImageRepr,
    matrix: &[(i8, i8, u8)],
    total_portions: u16,
    palette: &[Srgb],
) {
    // let (xdim, ydim) = image.dimensions();
    let ydim = image.len();
    let xdim = image.get(0).map(|row| row.len()).unwrap_or(0);

    if xdim == 0 || ydim == 0 {
        return;
    }

    for y in 0..ydim {
        for x in 0..xdim {
            let error = {
                let rgb = Srgb::from(image[y][x]).into_format::<f32>();
                let quantized = quantize_rgb(rgb, palette);
                image[y][x] = quantized.into_format().into();
                compute_rgb_error(rgb, quantized)
            };
    
            let error = (
                error.0 * 255.0,
                error.1 * 255.0,
                error.2 * 255.0,
            );
    
            for (x_off, y_off, portion) in matrix.iter() {
                let (x_err, y_err) = (
                    (x as i64 + *x_off as i64) as usize,
                    (y as i64 + *y_off as i64) as usize,
                );
    
                let pixel = image
                    .get_mut(y_err as usize)
                    .and_then(|row| row.get_mut(x_err as usize));
    
                if let Some(pixel) = pixel {
                    *pixel = [
                        (pixel[0] as f32 + (error.0 * *portion as f32) / total_portions as f32).clamp(0.0, 255.0) as u8,
                        (pixel[1] as f32 + (error.1 * *portion as f32) / total_portions as f32).clamp(0.0, 255.0) as u8,
                        (pixel[2] as f32 + (error.2 * *portion as f32) / total_portions as f32).clamp(0.0, 255.0) as u8,
                    ]
                }
            }
        }
    }

    // for x in 0..xdim {
    //     for y in 0..ydim {
    //         dbg!(image[y][x]);
    //     }
    // }
}

macro_rules! error_prop_rgb_fn {
    ($fn_name:ident, $matrix:expr, $portion_amnt:expr) => {
        pub fn $fn_name(image: &mut RgbImageRepr, palette: &[Srgb]) {
            error_propagate_through_pixels_rgb(image, $matrix, $portion_amnt, palette);
        }
    };
}


macro_rules! error_prop_mod {
    ($mod_name:ident, [$($matrix:tt)*] / $portion_amnt:expr) => {
        pub mod $mod_name {
            use palette::Srgb;
            use crate::{
                dither::{
                    error::{
                        error_propagate_through_pixels_rgb,
                        RgbImageRepr,
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
    use palette::Srgb;
    use crate::dither::error::{
        error_propagate_through_pixels_rgb,
        RgbImageRepr,
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