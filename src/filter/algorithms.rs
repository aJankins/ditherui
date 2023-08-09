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
        let mut image = self.into_rgb8();

        for (_, _, pixel) in image.enumerate_pixels_mut() {
            pixel.0 = pixel.0.run_through(algorithm)
        }

        DynamicImage::ImageRgb8(image)
    }
}

impl FilterInput for [u8; 3] {
    fn run_through(self, algorithm: &Algorithms) -> Self {
        match algorithm {
            Algorithms::RotateHue(degrees) => shift_hue(self, *degrees),
            Algorithms::Contrast(amount) => contrast(self, *amount),
            Algorithms::Brighten(amount) => brighten(self, *amount),
            Algorithms::Saturate(amount) => saturate(self, *amount),
            Algorithms::QuantizeHue(hues) => quantize_hue(self, hues),
            Algorithms::GradientMap(gradient) => gradient_map(self, gradient).map_or(self, |colour| colour.into_format().into()),
        }
    }
}

impl<'a, I: FilterInput> Effect<I> for Algorithms<'a> {
    fn affect(&self, item: I) -> I {
        item.run_through(self) 
    }
}