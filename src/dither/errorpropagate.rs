use image::{ImageBuffer, Rgb};

use crate::{
    pixel::{
        mono::{MonoPixel, ONE_BIT},
        rgb::RgbPixel,
    },
    utils::numops::map_to_2d,
};

pub fn error_propagate_through_pixels<const N: usize>(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    matrix: [(i8, i8, u8); N],
    total_portions: u16,
) {
    let (xdim, ydim) = image.dimensions();

    for i in xdim as u64..(xdim * ydim) as u64 {
        let (x, y) = map_to_2d(i, xdim);
        let error = {
            let pixel = image.get_pixel_mut(x, y);
            let mono = MonoPixel::from(&*pixel);
            let quantized = mono.quantize(ONE_BIT);
            pixel[0] = quantized.get();
            pixel[1] = quantized.get();
            pixel[2] = quantized.get();
            mono.get_error(&quantized)
        };

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
                pixel[0] = (pixel[0] as i32 + (error * *portion as i32) / total_portions as i32)
                    .clamp(0, 255) as u8;
                pixel[1] = (pixel[1] as i32 + (error * *portion as i32) / total_portions as i32)
                    .clamp(0, 255) as u8;
                pixel[2] = (pixel[2] as i32 + (error * *portion as i32) / total_portions as i32)
                    .clamp(0, 255) as u8;
            }
        }
    }
}

pub fn error_propagate_through_pixels_rgb<const N: usize>(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    matrix: [(i8, i8, u8); N],
    total_portions: u16,
    palette: &[RgbPixel],
) {
    let (xdim, ydim) = image.dimensions();

    for i in xdim as u64..(xdim * ydim) as u64 {
        let (x, y) = map_to_2d(i, xdim);
        let error = {
            let pixel = image.get_pixel_mut(x, y);
            let rgb = RgbPixel::from(&*pixel);
            let quantized = rgb.quantize(palette);
            (pixel[0], pixel[1], pixel[2]) = quantized.get_u8();
            rgb.get_error(&quantized)
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

macro_rules! error_prop_fn {
    ($fn_name:ident, $matrix:expr, $portion_amnt:expr) => {
        pub fn $fn_name(image: DynamicImage) -> DynamicImage {
            let mut rgb8_image = image.into_rgb8();
            error_propagate_through_pixels(&mut rgb8_image, $matrix, $portion_amnt);
            DynamicImage::ImageRgb8(rgb8_image)
        }
    };
}

macro_rules! error_prop_rgb_fn {
    ($fn_name:ident, $matrix:expr, $portion_amnt:expr) => {
        pub fn $fn_name(image: DynamicImage, palette: &[RgbPixel]) -> DynamicImage {
            let mut rgb8_image = image.into_rgb8();
            error_propagate_through_pixels_rgb(&mut rgb8_image, $matrix, $portion_amnt, palette);
            DynamicImage::ImageRgb8(rgb8_image)
        }
    };
}

macro_rules! error_prop_mod {
    ($mod_name:ident, {$($fn_name:ident $rgb_fn_name:ident [$($matrix:tt)*]{$matrix_name:ident, $error_amnt:expr, $portion_amnt:expr},)*}) => {
        pub mod $mod_name {
            use image::DynamicImage;
            use crate::{
                dither::{
                    errorpropagate::{
                        error_propagate_through_pixels,
                        error_propagate_through_pixels_rgb
                    },
                },
                pixel::rgb::RgbPixel
            };

            $(
                static $matrix_name: [(i8, i8, u8); $error_amnt] = [$($matrix)*];
            )*

            $(
                error_prop_fn!($fn_name, $matrix_name, $portion_amnt);
                error_prop_rgb_fn!($rgb_fn_name, $matrix_name, $portion_amnt);
            )*
        }
    };
}

error_prop_mod!(
    floydsteinberg,
    {
        floyd_steinberg_mono_dither
        floyd_steinberg_rgb_dither
        [
                                 (1, 0, 7),
            (-1, 1, 5),(0, 1, 3),(1, 1, 1),
        ]{MATRIX, 4, 16},
    }
);

error_prop_mod!(
    jarvisjudiceninke,
    {
        jarvis_judice_ninke_mono_dither
        jarvis_judice_ninke_rgb_dither
        [
                                            (1, 0, 7),(2, 0, 5),
            (-2, 1, 3),(-1, 1, 5),(0, 1, 7),(1, 1, 5),(2, 1, 3),
            (-2, 2, 1),(-1, 2, 3),(0, 2, 5),(1, 2, 3),(2, 2, 1),
        ]{MATRIX, 12, 48},
    }
);

error_prop_mod!(
    atkinson,
    {
        atkinson_mono_dither
        atkinson_rgb_dither
        [
                                 (1, 0, 1),(2, 0, 1),
            (-1, 1, 1),(0, 1, 1),(1, 1, 1),
                       (0, 2, 1)
        ]{MATRIX, 6, 8},
    }
);

error_prop_mod!(
    burkes,
    {
        burkes_mono_dither
        burkes_rgb_dither
        [
                                               (1, 0, 8), (2, 0, 4),
            (-2, 1, 2), (-1, 1, 4), (0, 1, 8), (1, 1, 4), (2, 1, 2),
        ]{MATRIX, 7, 32},
    }
);

error_prop_mod!(
    stucki,
    {
        stucki_mono_dither
        stucki_rgb_dither
        [
                                            (1, 0, 8),(2, 0, 4),
            (-2, 1, 2),(-1, 1, 4),(0, 1, 8),(1, 1, 4),(2, 1, 2),
            (-2, 2, 1),(-1, 2, 2),(0, 2, 4),(1, 2, 2),(2, 2, 1),
        ]{MATRIX, 12, 42},
    }
);

error_prop_mod!(
    sierra,
    {
        sierra_mono_dither
        sierra_rgb_dither
        [
                                            (1, 0, 5),(2, 0, 3),
            (-2, 1, 2),(-1, 1, 4),(0, 1, 5),(1, 1, 4),(2, 1, 2),
                       (-1, 2, 2),(0, 2, 3),(1, 2, 2),
        ]{MATRIX, 10, 32},

        two_row_sierra_mono_dither
        two_row_sierra_rgb_dither
        [
                                            (1, 0, 4),(2, 0, 3),
            (-2, 1, 1),(-1, 1, 2),(0, 1, 3),(1, 1, 2),(2, 1, 1),
        ]{TWO_ROW_MATRIX, 7, 16},

        sierra_lite_mono_dither
        sierra_lite_rgb_dither
        [
                                 (1, 0, 2),
            (-1, 1, 1),(0, 1, 1)
        ]{LITE_MATRIX, 3, 4},
    }
);
