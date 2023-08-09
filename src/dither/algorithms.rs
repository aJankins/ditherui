use image::{DynamicImage, GenericImageView, ImageBuffer};
use palette::Srgb;

use crate::{Effect, utils::numops::map_to_2d};

use super::{
    basic::basic_dither,
    bayer::bayer_dither,
    error::{
        floyd_steinberg, jarvis_judice_ninke, atkinson, burkes, stucki, sierra, RgbImageRepr
    },
};

/// Dithering algorithms. Each one accepts a *palette* - aka a list of `RgbPixel`s that colours should
/// be quantized to.
pub enum Algorithms<'a> {
    /// The basic error propagation method.
    ///
    /// Results in the worst quality output, but included for curiosity's sake
    Basic(&'a [Srgb]),
    /// Floyd Steinberg dithering.
    FloydSteinberg(&'a [Srgb]),
    /// Jarvis Judice Ninke dithering.
    JarvisJudiceNinke(&'a [Srgb]),
    /// Stucki dithering.
    Stucki(&'a [Srgb]),
    /// Atkinson dithering.
    Atkinson(&'a [Srgb]),
    /// Burkes dithering.
    Burkes(&'a [Srgb]),
    /// Sierra dithering.
    Sierra(&'a [Srgb]),
    /// Sierra two-row dithering.
    SierraTwoRow(&'a [Srgb]),
    /// Sierra lite dithering.
    SierraLite(&'a [Srgb]),
    /// Bayer / Ordered dithering.
    ///
    /// Accepts the matrix size. 1 results in no dithering, and 4+ is recommended.
    /// Isn't as accurate as the error propagation methods, but can be stylistically preferred.
    Bayer(usize, &'a [Srgb]),
}

pub trait DitherInput {
    fn run_through(self, algorithm: &Algorithms) -> Self;
}

impl DitherInput for DynamicImage {
    fn run_through(self, algorithm: &Algorithms) -> Self {
        let mut matrix = dynamic_image_to_2d_rgb_matrix(self);

        match algorithm {
            Algorithms::Basic(palette)                => basic_dither(&mut matrix, palette),
            Algorithms::FloydSteinberg(palette)       => floyd_steinberg::dither_rgb(&mut matrix, palette),
            Algorithms::JarvisJudiceNinke(palette)    => jarvis_judice_ninke::dither_rgb(&mut matrix, palette),
            Algorithms::Stucki(palette)               => stucki::dither_rgb(&mut matrix, palette),
            Algorithms::Atkinson(palette)             => atkinson::dither_rgb(&mut matrix, palette),
            Algorithms::Burkes(palette)               => burkes::dither_rgb(&mut matrix, palette),
            Algorithms::Sierra(palette)               => sierra::dither_rgb(&mut matrix, palette),
            Algorithms::SierraTwoRow(palette)         => sierra::two_row::dither_rgb(&mut matrix, palette),
            Algorithms::SierraLite(palette)           => sierra::lite::dither_rgb(&mut matrix, palette),
            Algorithms::Bayer(n, palette)     => bayer_dither(&mut matrix, *n, palette),
        }

        rgb_matrix_to_dynamic_image(matrix)
    }
}

impl <'a, I: DitherInput> Effect<I> for Algorithms<'a> {
    fn affect(&self, item: I) -> I {
        item.run_through(self)
    }
}

pub fn dynamic_image_to_2d_rgb_matrix(img: DynamicImage) -> RgbImageRepr {
    let (xs, ys) = img.dimensions();
    let (xs, ys) = (xs as usize, ys as usize);

    let img = img.into_rgb8();

    let mut img_matrix = vec![vec![[0_u8; 3]; xs]; ys];

    for (i, pixel) in img.pixels().into_iter().enumerate() {
       let (x, y) = map_to_2d(i, xs);
       img_matrix[y][x] = pixel.0;
    }

    img_matrix
}

pub fn rgb_matrix_to_dynamic_image(img: RgbImageRepr) -> DynamicImage {
    let ydim = img.len() as u32;
    let xdim = img.get(0).map(|row| row.len()).unwrap_or(0) as u32;

    let img = ImageBuffer::from_fn(xdim, ydim, |x, y| {
        image::Rgb(img[y as usize][x as usize])
    });

    DynamicImage::ImageRgb8(img)
}