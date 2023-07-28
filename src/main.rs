mod dither;
mod image;
mod utils;

pub use dither::algorithms::Algorithms as Dithers;
use ::image::ImageResult;

use crate::image::load_image;

fn main() -> ImageResult<()>{
    println!("Hello, world!");

    let image = load_image("data/original.png");
    
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

    Ok(())
}
