use image::DynamicImage;

use super::{
    basic::{basic_mono_dither, basic_colour_dither},
    stucki::stucki_mono_dither,
    atkinson::atkinson_mono_dither,
    burkes::burkes_mono_dither,
    floydsteinberg::floyd_steinberg_mono_dither,
    jarvisjudiceninke::jarvis_judice_ninke_mono_dither,
    sierra::{
        sierra_mono_dither,
        two_row_sierra_mono_dither,
        sierra_lite_mono_dither
    }, bayer::{bayer_mono_dither, bayer_dither}, pixel::RgbPixel};

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
            Self::Bayer(n, palette) => bayer_dither(image, *n, palette)
        }
    }
}