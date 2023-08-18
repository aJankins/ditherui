use std::marker::PhantomData;

use image::{ImageBuffer, Rgb};
use palette::Srgb;

use crate::{
    utils::{numops::map_to_2d, image::{get_dimensions_of_matrix, RgbImageRepr}},
    colour::utils::{quantize_rgb, ONE_BIT, compute_rgb_error, grayscale_rgb}, effect::Effect,
};

pub struct Base;
pub struct WithPalette;

pub trait PropagatorState {}
impl PropagatorState for Base {}
impl PropagatorState for WithPalette {}

pub struct ErrorPropagator<'name, 'matrix, S: PropagatorState> {
    pub name: &'name str,
    pub matrix: &'matrix [(i8, i8, u8)],
    pub portions: u16,
    palette: Option<Vec<Srgb>>,
    _phantom: PhantomData<S>,
}

impl<'a, 'b> ErrorPropagator<'a, 'b, Base> {
    pub const fn new(name: &'a str, matrix: &'b [(i8, i8, u8)], portions: u16) -> Self {
        ErrorPropagator {
            name,
            matrix,
            portions,
            palette: None,
            _phantom: PhantomData,
        }
    }
}

impl<'a, 'b, S: PropagatorState> ErrorPropagator<'a, 'b, S> {
    pub fn with_palette(&self, palette: Vec<Srgb>) -> ErrorPropagator<'a, 'b, WithPalette> {
        ErrorPropagator {
            name: self.name,
            matrix: self.matrix,
            portions: self.portions,
            palette: Some(palette),
            _phantom: PhantomData,
        }
    }
}

impl<'a, 'b> Effect<RgbImageRepr> for ErrorPropagator<'a, 'b, WithPalette> {
    fn affect(&self, mut image: RgbImageRepr) -> RgbImageRepr {
        let (xdim, ydim) = get_dimensions_of_matrix(&image);

        if xdim == 0 || ydim == 0 {
            return image;
        }

        for y in 0..ydim {
            for x in 0..xdim {
                let error = {
                    let rgb = Srgb::from(image[y][x]).into_format::<f32>();
                    let quantized = quantize_rgb(rgb, self.palette.as_ref().unwrap());
                    image[y][x] = quantized.into_format().into();
                    compute_rgb_error(rgb, quantized)
                };

                let error = (
                    error.0 * 255.0,
                    error.1 * 255.0,
                    error.2 * 255.0,
                );

                for (x_off, y_off, portion) in self.matrix.iter() {
                    let (x_err, y_err) = (
                        (x as i64 + *x_off as i64) as usize,
                        (y as i64 + *y_off as i64) as usize,
                    );

                    let pixel = image
                        .get_mut(y_err as usize)
                        .and_then(|row| row.get_mut(x_err as usize));

                    if let Some(pixel) = pixel {
                        *pixel = [
                            (pixel[0] as f32 + (error.0 * *portion as f32) / self.portions as f32).clamp(0.0, 255.0) as u8,
                            (pixel[1] as f32 + (error.1 * *portion as f32) / self.portions as f32).clamp(0.0, 255.0) as u8,
                            (pixel[2] as f32 + (error.2 * *portion as f32) / self.portions as f32).clamp(0.0, 255.0) as u8,
                        ]
                    }
                }
            }
        }
        image
    }
}

type ConstErrorPropagator = ErrorPropagator<'static, 'static, Base>;

pub const FLOYD_STEINBERG: ConstErrorPropagator = ErrorPropagator::new(
    "floyd-steinberg",
    &[
                            (1, 0, 7),
       (-1, 1, 5),(0, 1, 3),(1, 1, 1),
    ],
    16,
);

pub const JARVIS_JUDICE_NINKE: ConstErrorPropagator = ErrorPropagator::new(
    "jarvis-judice-ninke",
    &[
                                        (1, 0, 7),(2, 0, 5),
        (-2, 1, 3),(-1, 1, 5),(0, 1, 7),(1, 1, 5),(2, 1, 3),
        (-2, 2, 1),(-1, 2, 3),(0, 2, 5),(1, 2, 3),(2, 2, 1),
    ],
    48,
);

pub const ATKINSON: ConstErrorPropagator = ErrorPropagator::new(
    "atkinson",
    &[
                             (1, 0, 1),(2, 0, 1),
        (-1, 1, 1),(0, 1, 1),(1, 1, 1),
                   (0, 2, 1)
    ],
    8,
);

pub const BURKES: ConstErrorPropagator = ErrorPropagator::new(
    "burkes",
    &[
                                           (1, 0, 8), (2, 0, 4),
        (-2, 1, 2), (-1, 1, 4), (0, 1, 8), (1, 1, 4), (2, 1, 2),
    ],
    32,
);

pub const STUCKI: ConstErrorPropagator = ErrorPropagator::new(
    "stucki",
    &[
                                        (1, 0, 8),(2, 0, 4),
        (-2, 1, 2),(-1, 1, 4),(0, 1, 8),(1, 1, 4),(2, 1, 2),
        (-2, 2, 1),(-1, 2, 2),(0, 2, 4),(1, 2, 2),(2, 2, 1),
    ],
    42,
);

pub const SIERRA: ConstErrorPropagator = ErrorPropagator::new(
    "sierra",
    &[
                                        (1, 0, 5),(2, 0, 3),
        (-2, 1, 2),(-1, 1, 4),(0, 1, 5),(1, 1, 4),(2, 1, 2),
                   (-1, 2, 2),(0, 2, 3),(1, 2, 2),
    ],
    32,
);

pub const SIERRA_TWO_ROW: ConstErrorPropagator = ErrorPropagator::new(
    "sierra-two-row",
    &[
                                        (1, 0, 4),(2, 0, 3),
        (-2, 1, 1),(-1, 1, 2),(0, 1, 3),(1, 1, 2),(2, 1, 1),
    ],
    16,
);

pub const SIERRA_LITE: ConstErrorPropagator = ErrorPropagator::new(
    "sierra-lite",
    &[
                            (1, 0, 2),
        (-1, 1, 1),(0, 1, 1)
    ],
    4,
);