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
        .save("data/basicmono.png")?;
    
    Dithers::FloydSteinbergMono
        .dither(image.clone())
        .save("data/floydsteinbergmono.png")?;

    Dithers::JarvisJudiceNinkeMono
        .dither(image.clone())
        .save("data/jarvisjudiceninkemono.png")?;

    Dithers::StuckiMono
        .dither(image.clone())
        .save("data/stuckimono.png")?;

    Ok(())
}
