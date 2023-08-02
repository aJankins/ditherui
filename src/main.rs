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

    let gradient = [
        (RGB::BLACK, 0.00),
        (RGB::BLUE.add_error((-120, -120, -120)), 0.30),
        (RGB::MAGENTA.add_error((-80, -80, -80)), 0.60),
        (RGB::GOLD.add_error((30, 30, 30)), 0.70),
        (RGB::CYAN, 0.80),
        (RGB::WHITE, 1.00),
    ];

    let gen_gradient = |color: RgbPixel, modulus: u8| (-255..0)
        .into_iter()
        .filter(|i| i % modulus as i32 == 0)
        .map(|i| color.add_error((i, i, i)))
        .collect::<Vec<RgbPixel>>();

    let mut palette = [
        gen_gradient(RGB::ROSE, 25),
        gen_gradient(RGB::PURPLE, 25),
        gen_gradient(RGB::BLUE, 127),
        gen_gradient(RGB::CYAN, 50),
        gen_gradient(RGB::YELLOW, 127),
        gen_gradient(RGB::GOLD, 127),
        // gen_gradient(RGB::PINK, 64),
        vec![
            RGB::WHITE,
            RGB::BLACK,
        ],
    ].concat();
    
    let hue_palette = [40.0, 180.0, 330.0];

    load_image("data/input.png")?
        // .apply(Filter::Contrast(1.3))
        // .apply(Filter::Brighten(0.1))
        // .apply(Filter::GradientMap(&gradient))
        // .apply(Filter::QuantizeHue(&hue_palette))
        .apply(Filter::Contrast(0.9))
        .apply(Dither::Bayer(33, &palette))
        .apply(Filter::Contrast(3.0))
        .apply(Dither::Bayer(8, &palette))
        .apply(Filter::Saturate(0.5))
        .save("data/output.png")?;

    Ok(())
}