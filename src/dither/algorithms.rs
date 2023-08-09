use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use palette::Srgb;

use crate::{Effect, utils::numops::map_to_2d, EffectInput};

use super::{
    basic::basic_dither,
    bayer::bayer_dither,
    error::{
        floyd_steinberg, jarvis_judice_ninke, atkinson, burkes, stucki, sierra, RgbImageRepr
    },
};

/// Dithering algorithms. Each one accepts a *palette* - aka a list of `RgbPixel`s that colours should
/// be quantized to.
pub enum Dither<'a> {
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

impl<'a> EffectInput<Dither<'a>> for RgbImageRepr {
    fn run_through(&self, algorithm: &Dither) -> Self {
        let mut output = self.clone();
        match algorithm {
            Dither::Basic(palette)                => basic_dither(&mut output, palette),
            Dither::FloydSteinberg(palette)       => floyd_steinberg::dither_rgb(&mut output, palette),
            Dither::JarvisJudiceNinke(palette)    => jarvis_judice_ninke::dither_rgb(&mut output, palette),
            Dither::Stucki(palette)               => stucki::dither_rgb(&mut output, palette),
            Dither::Atkinson(palette)             => atkinson::dither_rgb(&mut output, palette),
            Dither::Burkes(palette)               => burkes::dither_rgb(&mut output, palette),
            Dither::Sierra(palette)               => sierra::dither_rgb(&mut output, palette),
            Dither::SierraTwoRow(palette)         => sierra::two_row::dither_rgb(&mut output, palette),
            Dither::SierraLite(palette)           => sierra::lite::dither_rgb(&mut output, palette),
            Dither::Bayer(n, palette)     => bayer_dither(&mut output, *n, palette),
        }
        output
    }
}

impl<'a> EffectInput<Dither<'a>> for ImageBuffer<Rgb<u8>, Vec<u8>> {
    fn run_through(&self, effect: &Dither<'a>) -> Self {
        let (xs, ys) = self.dimensions();
        let (xs, ys) = (xs as usize, ys as usize);
    
        let mut img_matrix = vec![vec![[0_u8; 3]; xs]; ys];
    
        for (i, pixel) in self.pixels().into_iter().enumerate() {
           let (x, y) = map_to_2d(i, xs);
           img_matrix[y][x] = pixel.0;
        }
    
        img_matrix = img_matrix.run_through(effect);

        let ydim = img_matrix.len() as u32;
        let xdim = img_matrix.get(0).map(|row| row.len()).unwrap_or(0) as u32;
    
        ImageBuffer::from_fn(xdim, ydim, |x, y| {
            image::Rgb(img_matrix[y as usize][x as usize])
        })
    }
}

impl<'a> EffectInput<Dither<'a>> for DynamicImage {
    fn run_through(&self, algorithm: &Dither) -> Self {
        DynamicImage::ImageRgb8(self.clone().into_rgb8().run_through(algorithm))
    }
}