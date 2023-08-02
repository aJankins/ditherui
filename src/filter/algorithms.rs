use image::DynamicImage;

use crate::{pixel::rgb::RgbPixel, ImageEffect};

/// Algorithms for applying filters to an image.
pub enum Algorithms<'a> {
    /// Rotates the hue based on the amount of degrees passed.
    RotateHue(f32),
    /// Modifies the contrast of the image.
    ///
    /// `1.0` means no change. Above adds contrast, below decreases it.
    Contrast(f32),
    /// Modifies the brightness of the image.
    ///
    /// This will directly affect the luminance of each pixel - which ranges between 0.0 and 1.0.
    /// Therefore `1.0` will turn the image white, and `-1.0` will turn the image black.
    Brighten(f32),
    /// Modifies the saturation of the image.
    ///
    /// This will directly affect the saturation of each pixel - which ranges between 0.0 and 1.0.
    /// Therefore `1.0` will maximally saturate each pixel, and `-1.0` will turn the image grayscale.
    Saturate(f32),
    /// Applies a gradient map to the image.
    ///
    /// The gradient map is defined as a slice of tuples containing the *colour* and its threshold.
    /// Each pixel in the image will be mapped to the gradient using its luminance value.
    ///
    /// The threshold must be between `0.0` and `1.0` - you can technically use other values but the results
    /// may be a bit weird.
    ///
    /// As an example, to turn an image grayscale you could pass the colour black at `0.0` and the colour
    /// white at `1.0`.
    GradientMap(&'a [(RgbPixel, f32)]),
    /// Quantizes the hue of each pixel to one of the hues passed.
    ///
    /// This ignores luminance or saturation and *only* changes the hue - useful for defining a colour
    /// scheme without losing detail.
    QuantizeHue(&'a [f32]),
}

impl<'a> ImageEffect<DynamicImage> for Algorithms<'a> {
    fn apply(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::RotateHue(degrees) => change_hue(image, *degrees),
            Self::Contrast(amount) => apply_contrast(image, *amount),
            Self::Brighten(amount) => apply_brightness(image, *amount),
            Self::Saturate(amount) => apply_saturation(image, *amount),
            Self::GradientMap(gradient) => apply_gradient_map(image, gradient),
            Self::QuantizeHue(hues) => apply_quantize_hue(image, hues),
        }
    }
}

fn change_hue(image: DynamicImage, degrees: f32) -> DynamicImage {
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let mut hsl = RgbPixel::from(&*pixel).to_hsl();
        hsl.add_hue(degrees);
        (pixel[0], pixel[1], pixel[2]) = hsl.to_rgb().get();
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

// contrast seems like it needs more research
fn apply_contrast(image: DynamicImage, amount: f32) -> DynamicImage {
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let (r, g, b) = RgbPixel::from(&*pixel).get();
        let new_r = (((r as i16 - 128) as f32 * amount) + 128.0).clamp(0.0, 255.0) as u8;
        let new_g = (((g as i16 - 128) as f32 * amount) + 128.0).clamp(0.0, 255.0) as u8;
        let new_b = (((b as i16 - 128) as f32 * amount) + 128.0).clamp(0.0, 255.0) as u8;
        (pixel[0], pixel[1], pixel[2]) = (new_r, new_g, new_b);
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

fn apply_brightness(image: DynamicImage, amount: f32) -> DynamicImage {
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let mut hsl = RgbPixel::from(&*pixel).to_hsl();
        hsl.add_luminance(amount);
        (pixel[0], pixel[1], pixel[2]) = hsl.to_rgb().get();
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

fn apply_saturation(image: DynamicImage, amount: f32) -> DynamicImage {
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let mut hsl = RgbPixel::from(&*pixel).to_hsl();
        hsl.add_saturation(amount);
        (pixel[0], pixel[1], pixel[2]) = hsl.to_rgb().get();
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

fn apply_gradient_map(image: DynamicImage, gradient: &[(RgbPixel, f32)]) -> DynamicImage {
    let mut rgb8_image = image.into_rgb8();

    let mut sorted = Vec::from(gradient.clone());
    sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for pixel in rgb8_image.pixels_mut() {
        let (_, _, l) = RgbPixel::from(&*pixel).to_hsl().get();

        let index = sorted.iter().position(|(_, threshold)| l < *threshold);
        if let Some(index) = index {
            let prev_col = sorted.get(index - 1);
            let curr_col = sorted.get(index);

            if prev_col.and(curr_col).is_some() {
                let (c_rgb, c_th) = curr_col.unwrap();
                let (p_rgb, p_th) = prev_col.unwrap();

                let c_dist = c_th - l;
                let p_dist = l - p_th;

                let c_ratio = 1.0 - (c_dist / (c_dist + p_dist));
                let p_ratio = 1.0 - (p_dist / (c_dist + p_dist));

                let (c_r, c_g, c_b) = c_rgb.get();
                let (p_r, p_g, p_b) = p_rgb.get();

                let (new_r, new_g, new_b) = (
                    (c_ratio * c_r as f32 + p_ratio * p_r as f32),
                    (c_ratio * c_g as f32 + p_ratio * p_g as f32),
                    (c_ratio * c_b as f32 + p_ratio * p_b as f32),
                );

                (pixel[0], pixel[1], pixel[2]) = (
                    new_r.clamp(0.0, 255.0).round() as u8,
                    new_g.clamp(0.0, 255.0).round() as u8,
                    new_b.clamp(0.0, 255.0).round() as u8,
                )
            } else if curr_col.is_some() {
                (pixel[0], pixel[1], pixel[2]) = curr_col.unwrap().0.get();
            }
        }
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

fn apply_quantize_hue(image: DynamicImage, hues: &[f32]) -> DynamicImage {
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let mut hsl = RgbPixel::from(&*pixel).to_hsl();
        hsl.quantize_hue(hues);
        (pixel[0], pixel[1], pixel[2]) = hsl.to_rgb().get();
    }

    DynamicImage::ImageRgb8(rgb8_image)
}
