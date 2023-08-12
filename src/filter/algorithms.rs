use image::{DynamicImage, ImageBuffer, Rgb, Frame, Rgba};
use palette::Srgb;

use crate::{Effect, EffectInput};

use super::raw::{contrast, gradient_map, quantize_hue, brighten, saturate, shift_hue, multiply_hue};

pub const CHROMA_BOUND: f32 = 128.0;

/// Algorithms for applying filters to an image.
pub enum Filter<'a> {
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
    /// Multiplies the hue of each pixel by the factor passed.
    MultiplyHue(f32),
}

// rgb pixel
impl<'a> EffectInput<Filter<'a>> for [u8; 3] {
    fn run_through(&self, algorithm: &Filter) -> Self {
        let clone = self.clone();
        match algorithm {
            Filter::RotateHue(degrees) => shift_hue(clone, *degrees),
            Filter::Contrast(amount) => contrast(clone, *amount),
            Filter::Brighten(amount) => brighten(clone, *amount),
            Filter::Saturate(amount) => saturate(clone, *amount),
            Filter::QuantizeHue(hues) => quantize_hue(clone, hues),
            Filter::GradientMap(gradient) => gradient_map(clone, gradient).map_or(clone, |colour| colour.into_format().into()),
            Filter::MultiplyHue(factor) => multiply_hue(clone, *factor),
        }
    }
}

// rgba pixel
impl<'a> EffectInput<Filter<'a>> for [u8; 4] {
    fn run_through(&self, algorithm: &Filter) -> Self {
        let [r, g, b, a] = self;

        let [r, g, b] = [*r, *g, *b].run_through(algorithm);

        [r, g, b, *a]
    }
}

impl<'a> EffectInput<Filter<'a>> for ImageBuffer<Rgb<u8>, Vec<u8>> {
    fn run_through(&self, effect: &Filter<'a>) -> Self {
        let mut output = self.clone();
        for (_, _, pixel) in output.enumerate_pixels_mut() {
            pixel.0 = pixel.0.run_through(effect)
        }
        output
    }
}

impl<'a> EffectInput<Filter<'a>> for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn run_through(&self, effect: &Filter<'a>) -> Self {
        let mut output = self.clone();
        for (_, _, pixel) in output.enumerate_pixels_mut() {
            pixel.0 = pixel.0.run_through(effect)
        }
        output
    }
}

impl<'a> EffectInput<Filter<'a>> for DynamicImage {
    fn run_through(&self, effect: &Filter) -> Self {
        match self {
            DynamicImage::ImageRgb8(img) 
                => DynamicImage::from(img.run_through(effect)),

            DynamicImage::ImageRgba8(img) 
                => DynamicImage::from(img.run_through(effect)),

            _ => DynamicImage::ImageRgb8(self.clone().into_rgb8().run_through(effect))
        }
    }
}

impl<'a> EffectInput<Filter<'a>> for Frame {
    fn run_through(&self, effect: &Filter<'a>) -> Self {
        let left = self.left();
        let top = self.top();
        let delay = self.delay();

        let new_buf = self.buffer().run_through(effect);
        Frame::from_parts(new_buf, left, top, delay)
    }
}