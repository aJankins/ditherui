use image::DynamicImage;
use palette::Srgb;

use crate::ImageEffect;

use super::{
    basic::{basic_colour_dither, basic_mono_dither},
    bayer::{bayer_dither, bayer_mono_dither},
    error::{
        floyd_steinberg, jarvis_judice_ninke, atkinson, burkes, stucki, sierra
    },
};

/// 1-bit dithering algorithms.
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

impl ImageEffect<DynamicImage> for MonoAlgorithms {
    fn apply(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::Basic             => basic_mono_dither(image),
            Self::FloydSteinberg    => floyd_steinberg::dither_1bit(image),
            Self::JarvisJudiceNinke => jarvis_judice_ninke::dither_1bit(image),
            Self::Stucki            => stucki::dither_1bit(image),
            Self::Atkinson          => atkinson::dither_1bit(image),
            Self::Burkes            => burkes::dither_1bit(image),
            Self::Sierra            => sierra::dither_1bit(image),
            Self::SierraTwoRow      => sierra::two_row::dither_1bit(image),
            Self::SierraLite        => sierra::lite::dither_1bit(image),
            Self::Bayer(n)  => bayer_mono_dither(image, *n),
        }
    }
}

impl<'a> ImageEffect<DynamicImage> for Algorithms<'a> {
    fn apply(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::Basic(palette)                => basic_colour_dither(image, palette),
            Self::FloydSteinberg(palette)       => floyd_steinberg::dither_rgb(image, palette),
            Self::JarvisJudiceNinke(palette)    => jarvis_judice_ninke::dither_rgb(image, palette),
            Self::Stucki(palette)               => stucki::dither_rgb(image, palette),
            Self::Atkinson(palette)             => atkinson::dither_rgb(image, palette),
            Self::Burkes(palette)               => burkes::dither_rgb(image, palette),
            Self::Sierra(palette)               => sierra::dither_rgb(image, palette),
            Self::SierraTwoRow(palette)         => sierra::two_row::dither_rgb(image, palette),
            Self::SierraLite(palette)           => sierra::lite::dither_rgb(image, palette),
            Self::Bayer(n, palette)     => bayer_dither(image, *n, palette),
        }
    }
}
