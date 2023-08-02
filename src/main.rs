use image_filters::{
    utils::{image::load_image, ImageFilterResult}, 
    pixel::{
        hsl::HslPixel,
        rgb::colours as RGB
    }, 
    prelude::*, hsl_gradient_map
};

// this file is essentially for testing / running the code, more than providing an actual reusable binary

fn main() -> ImageFilterResult<()> {
    let gradient = hsl_gradient_map![
        0.00 => sat: 0.0, lum: 0.0, hue: 0.0,
        0.30 => sat: 0.8, lum: 0.3, hue: 280.0,
        0.60 => sat: 0.8, lum: 0.6, hue: 200.0,
        0.80 => sat: 0.8, lum: 0.8, hue: 40.0,
        1.00 => sat: 0.0, lum: 1.0, hue: 260.0
    ];

    let palette = [
        RGB::CYAN.build_gradient(10),
        RGB::ROSE.build_gradient(10),
        RGB::PINK.build_gradient(10),
        RGB::GOLD.build_gradient(5),
        RGB::PURPLE.build_gradient(2),
        RGB::BLUE.build_gradient(2),
    ].concat();
    
    let _hue_palette: Vec<f32> = (0..=13).into_iter().map(|i| i as f32 * 30.0).collect();

    load_image("data/input.png")?
        .apply(Filter::GradientMap(&gradient))
        // .apply(Filter::QuantizeHue(&hue_palette))
        .apply(Filter::Contrast(1.3))
        .apply(Dither::Bayer(8, &palette))
        // .apply(Dither::Bayer(120, &palette))
        // .apply(Filter::Saturate(0.5))
        .save("data/output.png")?;

    Ok(())
}