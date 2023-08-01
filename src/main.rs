use image::ImageResult;
use image_filters::{
    utils::image::load_image, 
    colour::{pixel::hsl::HslPixel, palettes}, 
    prelude::*
};

// this file is essentially for testing / running the code, more than providing an actual reusable binary

fn main() -> ImageResult<()> {
    let image = load_image("data/input.png");

    let gradient = [
        (HslPixel::from((   0.0, 0.0, 0.0 )).to_rgb(), 0.00),
        (HslPixel::from(( 325.0, 0.5, 0.4 )).to_rgb(), 0.40),
        (HslPixel::from(( 300.0, 0.5, 0.5 )).to_rgb(), 0.50),
        (HslPixel::from(( 250.0, 0.5, 0.6 )).to_rgb(), 0.60),
        (HslPixel::from(( 200.0, 0.5, 0.7 )).to_rgb(), 0.70),
        (HslPixel::from(( 400.0, 0.5, 0.8 )).to_rgb(), 0.80),
        (HslPixel::from((   0.0, 0.0, 1.0 )).to_rgb(), 1.00),
    ];

    // let palette = [
    //     "FFFFFF",
    //     "003355", "0088AA", "00FFDD",
    //     "660055", "BB00AA", "FF00EE",
    //     "FFEE44",
    //     "000000",
    // ].map(|hex| hex.into());

    image
        // .apply(Colours::GradientMap(&gradient))
        // .apply(Colour::Brighten(0.1))
        // .apply(Colour::Contrast(1.3))
        // .apply(Dither::Bayer(8, &palette))
        .apply(Dither::Bayer(2, &palettes::WEB_SAFE))
        .save("data/output.png")?;

    Ok(())
}