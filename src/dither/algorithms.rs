use image::DynamicImage;

use crate::{pixel::rgb::RgbPixel, ImageEffect};

use super::{
    basic::{basic_colour_dither, basic_mono_dither},
    bayer::{bayer_dither, bayer_mono_dither},
    errorpropagate::{
        atkinson::{atkinson_mono_dither, atkinson_rgb_dither},
        burkes::{burkes_mono_dither, burkes_rgb_dither},
        floydsteinberg::{floyd_steinberg_mono_dither, floyd_steinberg_rgb_dither},
        jarvisjudiceninke::{jarvis_judice_ninke_mono_dither, jarvis_judice_ninke_rgb_dither},
        sierra::{
            sierra_lite_mono_dither, sierra_lite_rgb_dither, sierra_mono_dither, sierra_rgb_dither,
            two_row_sierra_mono_dither, two_row_sierra_rgb_dither,
        },
        stucki::{stucki_mono_dither, stucki_rgb_dither},
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
    Basic(&'a [RgbPixel]),
    /// Floyd Steinberg dithering.
    FloydSteinberg(&'a [RgbPixel]),
    /// Jarvis Judice Ninke dithering.
    JarvisJudiceNinke(&'a [RgbPixel]),
    /// Stucki dithering.
    Stucki(&'a [RgbPixel]),
    /// Atkinson dithering.
    Atkinson(&'a [RgbPixel]),
    /// Burkes dithering.
    Burkes(&'a [RgbPixel]),
    /// Sierra dithering.
    Sierra(&'a [RgbPixel]),
    /// Sierra two-row dithering.
    SierraTwoRow(&'a [RgbPixel]),
    /// Sierra lite dithering.
    SierraLite(&'a [RgbPixel]),
    /// Bayer / Ordered dithering.
    ///
    /// Accepts the matrix size. 1 results in no dithering, and 4+ is recommended.
    /// Isn't as accurate as the error propagation methods, but can be stylistically preferred.
    Bayer(usize, &'a [RgbPixel]),
}

impl ImageEffect<DynamicImage> for MonoAlgorithms {
    fn apply(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::Basic => basic_mono_dither(image),
            Self::FloydSteinberg => floyd_steinberg_mono_dither(image),
            Self::JarvisJudiceNinke => jarvis_judice_ninke_mono_dither(image),
            Self::Stucki => stucki_mono_dither(image),
            Self::Atkinson => atkinson_mono_dither(image),
            Self::Burkes => burkes_mono_dither(image),
            Self::Sierra => sierra_mono_dither(image),
            Self::SierraTwoRow => two_row_sierra_mono_dither(image),
            Self::SierraLite => sierra_lite_mono_dither(image),
            Self::Bayer(n) => bayer_mono_dither(image, *n),
        }
    }
}

impl<'a> ImageEffect<DynamicImage> for Algorithms<'a> {
    fn apply(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::Basic(palette) => basic_colour_dither(image, palette),
            Self::FloydSteinberg(palette) => floyd_steinberg_rgb_dither(image, palette),
            Self::JarvisJudiceNinke(palette) => jarvis_judice_ninke_rgb_dither(image, palette),
            Self::Stucki(palette) => stucki_rgb_dither(image, palette),
            Self::Atkinson(palette) => atkinson_rgb_dither(image, palette),
            Self::Burkes(palette) => burkes_rgb_dither(image, palette),
            Self::Sierra(palette) => sierra_rgb_dither(image, palette),
            Self::SierraTwoRow(palette) => two_row_sierra_rgb_dither(image, palette),
            Self::SierraLite(palette) => sierra_lite_rgb_dither(image, palette),
            Self::Bayer(n, palette) => bayer_dither(image, *n, palette),
        }
    }
}
