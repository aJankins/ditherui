mod dither;
mod image;
mod utils;

pub use dither::algorithms::Algorithms as Dithers;
use ::image::{ImageResult, DynamicImage};

use crate::{image::load_image, dither::{pixel::RgbPixel, palettes}};

fn main() -> ImageResult<()>{
    println!("Hello, world!");

    let image = load_image("data/original.png");

    // let palette: &[RgbPixel] = &[
    //     "FFFFFF",
    //     "003355", "0088AA", "00FFDD",
    //     "660055", "BB00AA", "FF00EE",
    //     "FFEE44",
    //     "000000",
    // ].map(|tuple| tuple.into());

    // let palette: &[RgbPixel] = &[
    //     "FFFFFF",
    //     "440055", "9900AA", "EE00FF", "FF00FF",
    //     "551100", "AA5500", "FFAA00", "FFFF00",
    //     "000000",
    // ].map(|tuple| tuple.into());

    mono(&image)?;
    colour_websafe(&image)?;
    colour_eightbit(&image)?;
    // colour(&image, palette)?;

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

fn colour_websafe(image: &DynamicImage) -> ImageResult<()> {
    colour(image, palettes::WEB_SAFE.as_ref(), Some("-web-safe"))
}

fn colour_eightbit(image: &DynamicImage) -> ImageResult<()> {
    colour(image, palettes::EIGHT_BIT.as_ref(), Some("-8-bit"))
}

fn colour(image: &DynamicImage, palette: &[RgbPixel], opt_postfix: Option<&str>) -> ImageResult<()> {
    let postfix = opt_postfix.unwrap_or("");

    Dithers::Basic(palette)
        .dither(image.clone())
        .save(format!("data/basic{}.png", postfix))?;

    Dithers::FloydSteinberg(palette)
        .dither(image.clone())
        .save(format!("data/floyd-steinberg{}.png", postfix))?;

    Dithers::JarvisJudiceNinke(palette)
        .dither(image.clone())
        .save(format!("data/jarvis-judice-ninke{}.png", postfix))?;

    Dithers::Stucki(palette)
        .dither(image.clone())
        .save(format!("data/stucki{}.png", postfix))?;

    Dithers::Atkinson(palette)
        .dither(image.clone())
        .save(format!("data/atkinson{}.png", postfix))?;

    Dithers::Burkes(palette)
        .dither(image.clone())
        .save(format!("data/burkes{}.png", postfix))?;

    Dithers::Sierra(palette)
        .dither(image.clone())
        .save(format!("data/sierra{}.png", postfix))?;

    Dithers::SierraTwoRow(palette)
        .dither(image.clone())
        .save(format!("data/sierra-two-row{}.png", postfix))?;

    Dithers::SierraLite(palette)
        .dither(image.clone())
        .save(format!("data/sierra-lite{}.png", postfix))?;

    Dithers::Bayer(2, palette)
        .dither(image.clone())
        .save(format!("data/bayer-2x2{}.png", postfix))?;

    Dithers::Bayer(4, palette)
        .dither(image.clone())
        .save(format!("data/bayer-4x4{}.png", postfix))?;

    Dithers::Bayer(8, palette)
        .dither(image.clone())
        .save(format!("data/bayer-8x8{}.png", postfix))?;

    Dithers::Bayer(16, palette)
        .dither(image.clone())
        .save(format!("data/bayer-16x16{}.png", postfix))?;

    Ok(())
}