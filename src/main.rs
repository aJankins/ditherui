use image_filters::{
    hsl_gradient_map,
    pixel::{hsl::HslPixel, rgb::colours as RGB},
    prelude::*,
    utils::{image::{load_image, resize_image, load_image_with_max_dim, load_image_from_url_with_max_dim}, ImageFilterResult},
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
    ]
    .concat();

    let link_to_image = "https://i.kym-cdn.com/photos/images/original/001/389/465/663.jpg";
    load_image_from_url_with_max_dim(link_to_image, 1080)?
        // .apply(Filter::GradientMap(&gradient))
        .apply(Filter::Contrast(1.3))
        .apply(Dither::Bayer(8, &palette))
        .save("data/output.png")?;

    Ok(())
}
