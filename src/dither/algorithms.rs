use image::DynamicImage;
use palette::Srgb;

use crate::Effect;

use super::{
    basic::{basic_colour_dither, basic_mono_dither},
    bayer::{bayer_dither, bayer_mono_dither},
    error::{
        floyd_steinberg, jarvis_judice_ninke, atkinson, burkes, stucki, sierra
    },
};

/// 1-bit dithering algorithms.
#[deprecated = "Use Algorithms with the ONE_BIT palette instead."]
pub enum MonoAlgorithms {
    /// The basic 1-bit error propagation method.
    ///
    /// Results in the worst quality output, but included for curiosity's sake.
    Basic,
    /// Floyd Steinberg 1-bit dithering.
    FloydSteinberg,
    /// Jarvis Judice Ninke 1-bit dithering.
    JarvisJudiceNinke,
    /// Stucki 1-bit dithering.
    Stucki,
    /// Atkinson 1-bit dithering.
    Atkinson,
    /// Burkes 1-bit dithering.
    Burkes,
    /// Sierra 1-bit dithering.
    Sierra,
    /// Sierra Two Row 1-bit dithering.
    SierraTwoRow,
    /// Sierra Lite 1-bit dithering.
    SierraLite,
    /// Bayer / Ordered 1-bit dithering.
    ///
    /// Accepts the matrix size. 1 results in no dithering, and 4+ is recommended.
    /// Isn't as accurate as the error propagation methods, but can be stylistically preferred.
    Bayer(usize),
}

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

pub trait MonoDitherInput {
    fn run_through(self, algorithm: &MonoAlgorithms) -> Self;
}

pub trait DitherInput {
    fn run_through(self, algorithm: &Algorithms) -> Self;
}

impl MonoDitherInput for DynamicImage {
    fn run_through(self, algorithm: &MonoAlgorithms) -> Self {
        match algorithm {
            MonoAlgorithms::Basic             => basic_mono_dither(self),
            MonoAlgorithms::FloydSteinberg    => floyd_steinberg::dither_1bit(self),
            MonoAlgorithms::JarvisJudiceNinke => jarvis_judice_ninke::dither_1bit(self),
            MonoAlgorithms::Stucki            => stucki::dither_1bit(self),
            MonoAlgorithms::Atkinson          => atkinson::dither_1bit(self),
            MonoAlgorithms::Burkes            => burkes::dither_1bit(self),
            MonoAlgorithms::Sierra            => sierra::dither_1bit(self),
            MonoAlgorithms::SierraTwoRow      => sierra::two_row::dither_1bit(self),
            MonoAlgorithms::SierraLite        => sierra::lite::dither_1bit(self),
            MonoAlgorithms::Bayer(n)  => bayer_mono_dither(self, *n),
        }
    }
}

impl DitherInput for DynamicImage {
    fn run_through(self, algorithm: &Algorithms) -> Self {
        match algorithm {
            Algorithms::Basic(palette)                => basic_colour_dither(self, palette),
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

impl <I: MonoDitherInput> Effect<I> for MonoAlgorithms {
    fn affect(&self, item: I) -> I {
        item.run_through(self)
    }
}

impl <'a, I: DitherInput> Effect<I> for Algorithms<'a> {
    fn affect(&self, item: I) -> I {
        item.run_through(self)
    }
}
