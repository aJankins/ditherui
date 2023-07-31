use image::DynamicImage;

use crate::colour::pixel::RgbPixel;

use super::{
    basic::{basic_mono_dither, basic_colour_dither}, bayer::{bayer_mono_dither, bayer_dither}, errorpropagate::{floydsteinberg::{floyd_steinberg_mono_dither, floyd_steinberg_rgb_dither}, jarvisjudiceninke::{jarvis_judice_ninke_mono_dither, jarvis_judice_ninke_rgb_dither}, stucki::{stucki_mono_dither, stucki_rgb_dither}, atkinson::{atkinson_mono_dither, atkinson_rgb_dither}, burkes::{burkes_mono_dither, burkes_rgb_dither}, sierra::{sierra_mono_dither, two_row_sierra_mono_dither, sierra_lite_mono_dither, sierra_rgb_dither, two_row_sierra_rgb_dither, sierra_lite_rgb_dither}}};

pub enum Algorithms<'a> {
    // Mono
    BasicMono,
    FloydSteinbergMono,
    JarvisJudiceNinkeMono,
    StuckiMono,
    AtkinsonMono,
    BurkesMono,
    SierraMono,
    SierraTwoRowMono,
    SierraLiteMono,
    BayerMono(usize),

    // Colour
    Basic(&'a [RgbPixel]),
    FloydSteinberg(&'a [RgbPixel]),
    JarvisJudiceNinke(&'a [RgbPixel]),
    Stucki(&'a [RgbPixel]),
    Atkinson(&'a [RgbPixel]),
    Burkes(&'a [RgbPixel]),
    Sierra(&'a [RgbPixel]),
    SierraTwoRow(&'a [RgbPixel]),
    SierraLite(&'a [RgbPixel]),
    Bayer(usize, &'a [RgbPixel]),
}

impl<'a> Algorithms<'a> {
    pub fn dither(&self, image: DynamicImage) -> DynamicImage {
        match self {
            // Mono
            Self::BasicMono => basic_mono_dither(image),
            Self::FloydSteinbergMono => floyd_steinberg_mono_dither(image),
            Self::JarvisJudiceNinkeMono => jarvis_judice_ninke_mono_dither(image),
            Self::StuckiMono => stucki_mono_dither(image),
            Self::AtkinsonMono => atkinson_mono_dither(image),
            Self::BurkesMono => burkes_mono_dither(image),
            Self::SierraMono => sierra_mono_dither(image),
            Self::SierraTwoRowMono => two_row_sierra_mono_dither(image),
            Self::SierraLiteMono => sierra_lite_mono_dither(image),
            Self::BayerMono(n) => bayer_mono_dither(image, *n),

            // Colour
            Self::Basic(palette) => basic_colour_dither(image, palette),
            Self::FloydSteinberg(palette) => floyd_steinberg_rgb_dither(image, palette),
            Self::JarvisJudiceNinke(palette) => jarvis_judice_ninke_rgb_dither(image, palette),
            Self::Stucki(palette) => stucki_rgb_dither(image, palette),
            Self::Atkinson(palette) => atkinson_rgb_dither(image, palette),
            Self::Burkes(palette) => burkes_rgb_dither(image, palette),
            Self::Sierra(palette) => sierra_rgb_dither(image, palette),
            Self::SierraTwoRow(palette) => two_row_sierra_rgb_dither(image, palette),
            Self::SierraLite(palette) => sierra_lite_rgb_dither(image, palette),
            Self::Bayer(n, palette) => bayer_dither(image, *n, palette)
        }
    }
}