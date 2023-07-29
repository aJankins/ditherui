mod dither;
mod image;
mod utils;

pub use dither::algorithms::Algorithms as Dithers;
use ::image::{ImageResult, DynamicImage};

use crate::{image::load_image, dither::{pixel::RgbPixel, palettes}};

fn main() -> ImageResult<()>{
    println!("Hello, world!");

    let image = load_image("data/original.png");

    let palette: &[RgbPixel] = &[
        "FFFFFF",

        "0000FF",
        "0055FF",
        "00AAFF",
        "00FFFF",

        "FF00FF",
        "AA00AA",
        "550055",

        "000000",
    ].map(|tuple| tuple.into());

    // mono(&image)?;
    // colour(&image, palettes::WEB_SAFE.as_ref())?;
    colour(&image, palette)?;

    Ok(())
}

fn mono(image: &DynamicImage) -> ImageResult<()> {
    // Dithers::BasicMono
    //     .dither(image.clone())
    //     .save("data/basic-mono.png")?;
    
    // Dithers::FloydSteinbergMono
    //     .dither(image.clone())
    //     .save("data/floyd-steinberg-mono.png")?;

    // Dithers::JarvisJudiceNinkeMono
    //     .dither(image.clone())
    //     .save("data/jarvis-judice-ninke-mono.png")?;

    // Dithers::StuckiMono
    //     .dither(image.clone())
    //     .save("data/stucki-mono.png")?;

    // Dithers::AtkinsonMono
    //     .dither(image.clone())
    //     .save("data/atkinson-mono.png")?;

    // Dithers::BurkesMono
    //     .dither(image.clone())
    //     .save("data/burkes-mono.png")?;

    // Dithers::SierraMono
    //     .dither(image.clone())
    //     .save("data/sierra-mono.png")?;

    // Dithers::SierraTwoRowMono
    //     .dither(image.clone())
    //     .save("data/sierra-two-row-mono.png")?;

    // Dithers::SierraLiteMono
    //     .dither(image.clone())
    //     .save("data/sierra-lite-mono.png")?;

    Dithers::BayerMono(2)
        .dither(image.clone())
        .save("data/bayer-2x2-mono.png")?;

    Dithers::BayerMono(4)
        .dither(image.clone())
        .save("data/bayer-4x4-mono.png")?;

    Dithers::BayerMono(8)
        .dither(image.clone())
        .save("data/bayer-8x8-mono.png")?;

    Dithers::BayerMono(16)
        .dither(image.clone())
        .save("data/bayer-16x16-mono.png")?;

    Ok(())
}

fn colour(image: &DynamicImage, palette: &[RgbPixel]) -> ImageResult<()> {
    // Dithers::Basic(palette)
    //     .dither(image.clone())
    //     .save("data/basic.png")?;

    // Dithers::Bayer(2, palette)
    //     .dither(image.clone())
    //     .save("data/bayer-2x2.png")?;

    // Dithers::Bayer(4, palette)
    //     .dither(image.clone())
    //     .save("data/bayer-4x4.png")?;

    // Dithers::Bayer(8, palette)
    //     .dither(image.clone())
    //     .save("data/bayer-8x8.png")?;

    Dithers::Bayer(16, palette)
        .dither(image.clone())
        .save("data/bayer-16x16.png")?;

    Ok(())
}