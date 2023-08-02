use image_filters::{
    utils::{image::load_image, ImageFilterResult}, 
    pixel::{hsl::HslPixel, rgb::{colours as RGB, RgbPixel}}, 
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

    let mut palette = [
        RGB::CYAN.build_gradient(10),
        RGB::ROSE.build_gradient(10),
        RGB::PINK.build_gradient(10),
        RGB::GOLD.build_gradient(5),
        RGB::PURPLE.build_gradient(2),
        RGB::BLUE.build_gradient(2),
    ].concat();
    
    let hue_palette = [40.0, 180.0, 330.0];

    load_image("data/input.png")?
        // .apply(Filter::GradientMap(&gradient))
        // .apply(Filter::QuantizeHue(&hue_palette))
        // .apply(Filter::Contrast(2.0))
        .apply(Dither::Bayer(8, &palette))
        // .apply(Dither::Bayer(120, &palette))
        // .apply(Filter::Saturate(0.5))
        .save("data/output.png")?;

    Ok(())
}