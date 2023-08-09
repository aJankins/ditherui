use image::DynamicImage;
use palette::Srgb;

use crate::Effect;

use super::raw::{contrast, gradient_map, quantize_hue, brighten, saturate, shift_hue};

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
}

pub trait FilterInput {
    fn run_through(self, algorithm: &Algorithms) -> Self;
}

impl FilterInput for DynamicImage {
    fn run_through(self, algorithm: &Algorithms) -> Self {
        match algorithm {
            Algorithms::RotateHue(degrees) => dynamic_image_shift_hue(self, *degrees),
            Algorithms::Contrast(amount) => dynamic_image_contrast(self, *amount),
            Algorithms::Brighten(amount) => dynamic_image_brighten(self, *amount),
            Algorithms::Saturate(amount) => dynamic_image_saturate(self, *amount),
            Algorithms::QuantizeHue(hues) => dynamic_image_quantize_hue(self, hues),
            Algorithms::GradientMap(gradient) => dynamic_image_gradient_map(self, gradient),
        }
    }
}

impl<'a, I: FilterInput> Effect<I> for Algorithms<'a> {
    fn affect(&self, item: I) -> I {
        item.run_through(self) 
    }
}

macro_rules! dynamic_image_effect {
    ($fn_name:ident($($param:ident: $type:ty),+) -> $effect_fn:ident) => {
        fn $fn_name (image: DynamicImage, $($param: $type),+) -> DynamicImage {
            let mut image = image.into_rgb8();

            for (_, _, pixel) in image.enumerate_pixels_mut() {
                pixel.0 = $effect_fn (pixel.0, $($param),+);
            }

            DynamicImage::ImageRgb8(image)
        }
    };
}

// dynamic image effects

dynamic_image_effect!(dynamic_image_contrast(amount: f32) -> contrast);
dynamic_image_effect!(dynamic_image_quantize_hue(hues: &[f32]) -> quantize_hue);
dynamic_image_effect!(dynamic_image_brighten(amount: f32) -> brighten);
dynamic_image_effect!(dynamic_image_saturate(amount: f32) -> saturate);
dynamic_image_effect!(dynamic_image_shift_hue(degrees: f32) -> shift_hue);

fn dynamic_image_gradient_map(image: DynamicImage, gradient: &[(Srgb, f32)]) -> DynamicImage {
    let mut image = image.into_rgb8();

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        let rgb = gradient_map(pixel.0, gradient);

        if let Some(rgb) = rgb {
            pixel.0 = rgb.into_format().into();
        }


    }

    DynamicImage::ImageRgb8(image)
}