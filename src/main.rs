use image_filters::{
    utils::{image::load_image, ImageFilterResult}, 
    pixel::hsl::HslPixel, 
    prelude::*
};

// this file is essentially for testing / running the code, more than providing an actual reusable binary

fn main() -> ImageFilterResult<()> {
    let gradient = [
        (HslPixel::from((   0.0, 0.0, 0.0 )).to_rgb(), 0.00),
        (HslPixel::from(( 340.0, 0.8, 0.4 )).to_rgb(), 0.30),
        (HslPixel::from(( 180.0, 0.8, 0.5 )).to_rgb(), 0.60),
        (HslPixel::from((  40.0, 0.8, 0.8 )).to_rgb(), 0.80),
        (HslPixel::from((   0.0, 0.0, 1.0 )).to_rgb(), 1.00),
    ];

    let palette = [
        "FFFFFF",
        "FF00BB", "880066",
        "00FFFF", "008888",
        "FFBB00", "886600",
        "000000",
    ].map(|hex| hex.into());
    
    let hue_palette = [40.0, 180.0, 330.0];

    load_image("data/input.png")?
        .apply(Filter::GradientMap(&gradient))
        .apply(Filter::QuantizeHue(&hue_palette))
        .apply(Dither::Bayer(8, &palette))
        .save("data/output.png")?;

    Ok(())
}