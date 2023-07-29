mod dither;
mod image;
mod utils;

pub use dither::algorithms::Algorithms as Dithers;
use ::image::{ImageResult, DynamicImage};

use crate::{image::load_image, dither::pixel::RgbPixel};

fn main() -> ImageResult<()>{
    println!("Hello, world!");

    let image = load_image("data/original.png");

    let palette: &[RgbPixel] = &[
        (255, 255, 255),
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (255, 255, 0),
        (0, 255, 255),
        (255, 0, 255),

        (128, 128, 0),
        (0, 128, 128),
        (128, 0, 128),

        (0, 0, 0)
    ].map(|tuple| tuple.into());

    // let mut palette = vec![];

    // for r in 0..5 {
    //     for g in 0..5 {
    //         for b in 0..5 {
    //             palette.push((r*50, g*50, b*50))
    //         }
    //     }
    // }

    mono(&image)?;
    colour(&image, palette)?;

    // let closest = get_closest_color_to((100,150,35), palette);
    // println!("CLOSEST COLOUR: {:#?}", closest);

    Ok(())
}

fn mono(image: &DynamicImage) -> ImageResult<()> {
    Dithers::BasicMono
        .dither(image.clone())
        .save("data/basic-mono.png")?;
    
    Dithers::FloydSteinbergMono
        .dither(image.clone())
        .save("data/floyd-steinberg-mono.png")?;

    Dithers::JarvisJudiceNinkeMono
        .dither(image.clone())
        .save("data/jarvis-judice-ninke-mono.png")?;

    Dithers::StuckiMono
        .dither(image.clone())
        .save("data/stucki-mono.png")?;

    Dithers::AtkinsonMono
        .dither(image.clone())
        .save("data/atkinson-mono.png")?;

    Dithers::BurkesMono
        .dither(image.clone())
        .save("data/burkes-mono.png")?;

    Dithers::SierraMono
        .dither(image.clone())
        .save("data/sierra-mono.png")?;

    Dithers::SierraTwoRowMono
        .dither(image.clone())
        .save("data/sierra-two-row-mono.png")?;

    Dithers::SierraLiteMono
        .dither(image.clone())
        .save("data/sierra-lite-mono.png")?;

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
    Dithers::Basic(palette)
        .dither(image.clone())
        .save("data/basic.png")?;

    Ok(())
}