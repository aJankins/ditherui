use image::DynamicImage;

use crate::dither::pixel::RgbPixel;



pub enum Algorithms<'a> {
    Hue(u16),
    Contrast(u16),
    Brightness(u16),
    Saturate(u16),
    GradientMap(&'a [(RgbPixel, f32)]),
}

impl<'a> Algorithms<'a> {
    pub fn apply(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::Hue(degrees) => change_hue(image, *degrees),
            Self::Contrast(amount) => apply_contrast(image, *amount),
            Self::Brightness(amount) => apply_brightness(image, *amount),
            Self::Saturate(amount) => apply_saturation(image, *amount),
            Self::GradientMap(gradient) => apply_gradient_map(image, gradient),
        }
    }
}

fn change_hue(image: DynamicImage, degrees: u16) -> DynamicImage {
    unimplemented!()
}

fn apply_contrast(image: DynamicImage, amount: u16) -> DynamicImage {
    unimplemented!()
}

fn apply_brightness(image: DynamicImage, amount: u16) -> DynamicImage {
    unimplemented!()
}

fn apply_saturation(image: DynamicImage, amount: u16) -> DynamicImage {
    unimplemented!()
}

fn apply_gradient_map(image: DynamicImage, gradient: &[(RgbPixel, f32)]) -> DynamicImage {
    unimplemented!()
}