use image::DynamicImage;

use crate::colour::pixel::hsl::HslPixel;

use super::pixel::rgb::{RgbPixel, self};
pub enum Algorithms<'a> {
    RotateHue(f32),
    Contrast(u16),
    Brighten(f32),
    Saturate(f32),
    GradientMap(&'a [(RgbPixel, f32)]),
    NormalizeMono,
}

impl<'a> Algorithms<'a> {
    pub fn apply(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::RotateHue(degrees) => change_hue(image, *degrees),
            Self::Contrast(amount) => apply_contrast(image, *amount),
            Self::Brighten(amount) => apply_brightness(image, *amount),
            Self::Saturate(amount) => apply_saturation(image, *amount),
            Self::GradientMap(gradient) => apply_gradient_map(image, gradient),
            Self::NormalizeMono => apply_normalize_mono(image),
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

fn apply_contrast(image: DynamicImage, amount: u16) -> DynamicImage {
    unimplemented!()
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
    let (xs, ys) = rgb8_image.dimensions();
    println!("Dimensions of image: {}x{}", xs, ys);

    for pixel in rgb8_image.pixels_mut() {
        let mut hsl = RgbPixel::from(&*pixel).to_hsl();
        hsl.add_saturation(amount);
        (pixel[0], pixel[1], pixel[2]) = hsl.to_rgb().get();
    }
    
    DynamicImage::ImageRgb8(rgb8_image)
}

fn apply_gradient_map(image: DynamicImage, gradient: &[(RgbPixel, f32)]) -> DynamicImage {
    unimplemented!()
}

fn apply_normalize_mono(image: DynamicImage) -> DynamicImage {
    unimplemented!()
}