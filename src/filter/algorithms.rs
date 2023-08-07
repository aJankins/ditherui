use image::DynamicImage;
use palette::{cast, Oklch, Srgb, IntoColor, SetHue, ShiftHue, FromColor, Lighten, Darken, Lch, rgb::Rgb};

use crate::{ImageEffect};

use super::direct::{contrast, gradient_map, rgb_to_srgb, srgb_to_rgb, quantize_hue};

pub const CHROMA_BOUND: f32 = 128.0;

/// Algorithms for applying filters to an image.
pub enum Algorithms<'a> {
    /// Rotates the hue based on the amount of degrees passed.
    RotateHue(f32),
    /// Modifies the contrast of the image.
    ///
    /// - `>1.0`: adds contrast to image.
    /// - `0.0 ~ 1.0`: reduces contrast to image.
    /// - `<0.0`: starts inverting the image - with `-1.0` being total inversion.
    Contrast(f32),
    /// Modifies the brightness of the image.
    ///
    /// This value can range from `-1.0`, which turns all pixels black, and `1.0`, which makes
    /// all pixels white.
    Brighten(f32),
    /// Modifies the saturation of the image.
    ///
    /// This value can range from `-1.0`, which removes all saturation, and `1.0`, which maximizes
    /// all saturation.
    /// 
    /// Internally, `Saturate(1.0)` would mean setting each pixel to `128.0 chroma` in LCH terms -
    /// despite Chroma being technically unbounded.
    /// 
    /// This may change in the future.
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
    GradientMap(&'a [(Srgb, f32)]),
    /// Quantizes the hue of each pixel to one of the hues passed.
    ///
    /// This *only* changes the hue - useful for defining a colour
    /// scheme without losing luminance/saturation detail.
    QuantizeHue(&'a [f32]),
    DEBUG,
}

impl<'a> ImageEffect<DynamicImage> for Algorithms<'a> {
    fn apply(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::RotateHue(degrees) => shift_hue(image, *degrees),
            Self::Contrast(amount) => apply_contrast(image, *amount),
            Self::Brighten(amount) => brighten(image, *amount),
            Self::Saturate(amount) => saturate(image, *amount),
            Self::GradientMap(gradient) => apply_gradient_map(image, gradient),
            Self::QuantizeHue(hues) => apply_quantize_hue(image, hues),
            Self::DEBUG => _debug_filter(image),
        }
    }
}

fn apply_contrast(image: DynamicImage, amount: f32) -> DynamicImage {
    let mut image = image.into_rgb8();

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        pixel.0 = contrast(pixel.0, amount)
    }

    DynamicImage::ImageRgb8(image)
}

fn apply_gradient_map(image: DynamicImage, gradient: &[(Srgb, f32)]) -> DynamicImage {
    let mut image = image.into_rgb8();

    let mut sorted = Vec::from(gradient.clone());
    sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        let rgb = gradient_map(pixel.0, gradient);

        if let Some(rgb) = rgb {
            pixel.0 = rgb.into_format().into();
        }


    }

    DynamicImage::ImageRgb8(image)
}

fn apply_quantize_hue(image: DynamicImage, hues: &[f32]) -> DynamicImage {
    let mut image = image.into_rgb8();

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        pixel.0 = quantize_hue(pixel.0, hues);
    }

    DynamicImage::ImageRgb8(image)
}

fn brighten(image: DynamicImage, amount: f32) -> DynamicImage {
    let mut image = image.into_rgb8();

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        let color = Srgb::from(pixel.0).into_format::<f32>();
        let mut color = Lch::from_color(color);

        if amount >= 0.0 {
            color = color.lighten(amount);
        } else {
            color = color.darken(amount.abs());
        };

        pixel.0 = Srgb::from_color(color).into_format().into();
    }

    DynamicImage::ImageRgb8(image)
}

fn saturate(image: DynamicImage, amount: f32) -> DynamicImage {
    let mut image = image.into_rgb8();

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        let color = Srgb::from(pixel.0).into_format::<f32>();
        let mut color = Lch::from_color(color);

        color.chroma = if amount >= 0.0 {
            color.chroma + (CHROMA_BOUND - color.chroma) * amount
        } else {
            color.chroma + (color.chroma) * amount
        };
        pixel.0 = Srgb::from_color(color).into_format().into();
    }

    DynamicImage::ImageRgb8(image)
}

fn shift_hue(image: DynamicImage, degrees: f32) -> DynamicImage {
    let mut image = image.into_rgb8();

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        let color = Srgb::from(pixel.0).into_format::<f32>();
        let mut color = Lch::from_color(color);
        color = color.shift_hue(degrees);
        pixel.0 = Srgb::from_color(color).into_format().into();
    }

    DynamicImage::ImageRgb8(image)
}

// debug code

fn _debug_filter(image: DynamicImage) -> DynamicImage {
    let _ = image.save("data/original.png");
    image
}