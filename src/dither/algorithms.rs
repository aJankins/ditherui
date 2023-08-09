use image::DynamicImage;
use palette::Srgb;

use crate::Effect;

use super::{
    basic::basic_dither,
    bayer::bayer_dither,
    error::{
        floyd_steinberg, jarvis_judice_ninke, atkinson, burkes, stucki, sierra
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
        match algorithm {
            Algorithms::Basic(palette)                => basic_dither(self, palette),
            Algorithms::FloydSteinberg(palette)       => floyd_steinberg::dither_rgb(self, palette),
            Algorithms::JarvisJudiceNinke(palette)    => jarvis_judice_ninke::dither_rgb(self, palette),
            Algorithms::Stucki(palette)               => stucki::dither_rgb(self, palette),
            Algorithms::Atkinson(palette)             => atkinson::dither_rgb(self, palette),
            Algorithms::Burkes(palette)               => burkes::dither_rgb(self, palette),
            Algorithms::Sierra(palette)               => sierra::dither_rgb(self, palette),
            Algorithms::SierraTwoRow(palette)         => sierra::two_row::dither_rgb(self, palette),
            Algorithms::SierraLite(palette)           => sierra::lite::dither_rgb(self, palette),
            Algorithms::Bayer(n, palette)     => bayer_dither(self, *n, palette),
        }
    }
}

impl <'a, I: DitherInput> Effect<I> for Algorithms<'a> {
    fn affect(&self, item: I) -> I {
        item.run_through(self)
    }
}
