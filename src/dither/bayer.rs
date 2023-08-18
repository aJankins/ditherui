use image::{DynamicImage, Pixel};
use ndarray::{concatenate, Array, Axis, Dim};
use palette::Srgb;

use crate::{utils::{numops::average, image::{RgbImageRepr, RgbPixelRepr}}, colour::utils::quantize_rgb, effect::Effect};

/// Represents the _ordered_ method of dithering. Compared to error propagation, this method is less accurate - however it
/// results in a pattern that can be visually appealing.
/// 
/// In addition it only modifies each pixel on its own without needing to simultaneously touch/affect other pixels, making it 
/// easily possible to parallellize.
pub struct Bayer {
    matrix_size: usize,
    palette: Vec<Srgb>,
}

impl Bayer {

    /// Creates a new `Bayer` ditherer with the given matrix size.
    pub fn new(matrix_size: usize, palette: Vec<Srgb>) -> Self {
        Self { matrix_size, palette }
    }

    /// Creates a clone of the ditherer with a different matrix size.
    pub fn with_matrix_size(&self, matrix_size: usize) -> Self {
        Self { matrix_size, palette: self.palette.clone() }
    }

    fn dither_matrix(n: usize) -> Array<f64, Dim<[usize; 2]>> {
        if n == 1 {
            return Array::<f64, _>::zeros((1, 1));
        }

        let nested_matrix = Self::dither_matrix(n / 2);
        let multiplier = n.pow(2) as f64;

        let first = multiplier * nested_matrix.clone();
        let second = multiplier * nested_matrix.clone() + 2.;
        let third = multiplier * nested_matrix.clone() + 3.;
        let fourth = multiplier * nested_matrix.clone() + 1.;

        let first_col = concatenate(Axis(0), &[first.view(), third.view()]).unwrap();
        let second_col = concatenate(Axis(0), &[second.view(), fourth.view()]).unwrap();

        (1. / multiplier) * concatenate(Axis(1), &[first_col.view(), second_col.view()]).unwrap()
    }
}

impl Effect<RgbImageRepr> for Bayer {
    fn affect(&self, mut image: RgbImageRepr) -> RgbImageRepr {
        let matrix = Self::dither_matrix(self.matrix_size);

        let ydim = image.len();
        let xdim = image.get(0).map(|row| row.len()).unwrap_or(0);

        for y in 0..ydim {
            for x in 0..xdim {
                let mut color = Srgb::from(image[y][x]).into_format::<f32>();
        
                let offset = (1.0 / 3.0)
                    * (matrix
                        .get((x % self.matrix_size, y % self.matrix_size))
                        .unwrap_or(&0.0)
                        - 0.5) as f32;
        
                color.red = color.red + offset;
                color.blue = color.blue + offset;
                color.green = color.green + offset;
        
                image[y][x] = quantize_rgb(color, &self.palette).into_format().into();
            }
        }

        image
    }
}