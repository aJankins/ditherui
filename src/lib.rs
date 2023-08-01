//! This crate provides multiple effects that can be applied on an image.
//! 
//! Currently there's two classes of effects:
//! 
//! 1. **Dithering** - Limiting the colour palette of a given image while still
//!    retaining more detail than a purely quantized approach would.
//! 2. **Filters** - Some more common effects applied on an image, such as brightness,
//!    contrast, gradient mapping, and more.
//! 
//! This crate assumes that you are using the `image` crate for processing - as all these
//! algorithms work specifically with the `DynamicImage` struct (for now).
//! 
//! The *prelude* comes with nice re-exports of the algorithms under `MonoDither`, `Dither` and `Filter`
//! - in addition to the `ImageEffect` trait *(applied to all algorithms)* and `AdjustableImage`
//! trait *(implemented for `DynamicImage`)*.
//! 
//! Importing the `AdjustableImage` trait (manually or via the prelude) will allow you to call
//! `.apply` on a `DynamicImage` directly. For comparison, here are both approaches for applying
//! an effect:
//! 
//! ```
//! Colours::Brighten( 0.2).apply(image); // without trait
//! image.apply(Colours::Brighten( 0.2)); // with trait
//! ```
//! 
//! The benefit of this mostly comes when chaining effects together, and it's included mostly
//! just for better code ergonomics.

use image::DynamicImage;

/// Contains multiple algorithms for dithering an image - both in 1-bit and RGB variants.
pub mod dither;

/// Filters that can be applied to the image - such as brightness, contrast, and more.
pub mod filter;

/// Pixel utilities. Facilitates certain functionality such as colour difference and conversion between spaces.
pub mod pixel;

/// Utilities. Mostly just for the test cases - will probably be removed.
pub mod utils;

/// Prelude for including the useful elements from the library - including traits and algorithms.
pub mod prelude {
    pub use crate::dither::algorithms::MonoAlgorithms as MonoDither;
    pub use crate::dither::algorithms::Algorithms as Dither;
    pub use crate::filter::algorithms::Algorithms as Filter;
    pub use crate::AdjustableImage;
    pub use crate::ImageEffect;
}

/// Defines an effect that can be applied onto an image.
pub trait ImageEffect<T: ?Sized> {
    fn apply(&self, image: T) -> T;
}

/// Allows the implemented struct to use `.apply` directly.
pub trait AdjustableImage {
    fn apply(self, effect: impl ImageEffect<Self>) -> Self;
}

impl AdjustableImage for DynamicImage {
    fn apply(self, effect: impl ImageEffect<Self>) -> Self {
        effect.apply(self)
    }
}

#[cfg(test)]
mod test {
    use image::{ImageResult, DynamicImage};

    use crate::{
        utils::{image::load_image, ImageFilterResult},
        pixel::rgb::RgbPixel,
        prelude::*, dither::palettes
    };

    #[test]
    fn dither_test() -> ImageFilterResult<()> {    
        let image = load_image("data/input.png")?;
    
        let palette: &[RgbPixel] = &[
            "FFFFFF",
            "003355", "0088AA", "00FFDD",
            "660055", "BB00AA", "FF00EE",
            "FFEE44",
            "000000",
        ].map(|tuple| tuple.into());
        
        mono(&image)?;
        colour_websafe(&image)?;                              // takes a long time due to large palette
        colour_eightbit(&image)?;                             // significantly faster
        colour(&image, palette, Some("-custom-palette"))?;    // custom palettes, uncomment a palette above for examples
    
        Ok(())
    }

    #[test]
    fn colour_effects_test() -> ImageFilterResult<()> {
        let image = load_image("data/original.png")?;

        image.clone().apply(Filter::RotateHue(180.0)).save("data/colour/rotate-hue-180.png")?;
        image.clone().apply(Filter::Brighten( 0.2)).save("data/colour/brighten+0.2.png")?;
        image.clone().apply(Filter::Brighten(-0.2)).save("data/colour/brighten-0.2.png")?;
        image.clone().apply(Filter::Saturate( 0.2)).save("data/colour/saturate+0.2.png")?;
        image.clone().apply(Filter::Saturate(-0.2)).save("data/colour/saturate-0.2.png")?;
        image.clone().apply(Filter::Contrast( 0.5)).save("data/colour/contrast.0.5.png")?;
        image.clone().apply(Filter::Contrast( 1.5)).save("data/colour/contrast.1.5.png")?;

        let gradient_map = [
            ("000000".into(), 0.00),
            ("0000FF".into(), 0.25),
            ("FF0000".into(), 0.50),
            ("00FF00".into(), 0.75),
            ("FFFFFF".into(), 1.00),
        ];

        image.clone().apply(Filter::GradientMap(&gradient_map)).save("data/colour/gradient-mapped.png")?;

        Ok(())
    }

    fn mono(image: &DynamicImage) -> ImageResult<()> {
        image.clone().apply(MonoDither::Basic).save("data/dither/basic-mono.png")?;
        image.clone().apply(MonoDither::FloydSteinberg).save("data/dither/floyd-steinberg-mono.png")?;
        image.clone().apply(MonoDither::JarvisJudiceNinke).save("data/dither/jarvis-judice-ninke-mono.png")?;
        image.clone().apply(MonoDither::Stucki).save("data/dither/stucki-mono.png")?;
        image.clone().apply(MonoDither::Atkinson).save("data/dither/atkinson-mono.png")?;
        image.clone().apply(MonoDither::Burkes).save("data/dither/burkes-mono.png")?;
        image.clone().apply(MonoDither::Sierra).save("data/dither/sierra-mono.png")?;
        image.clone().apply(MonoDither::SierraTwoRow).save("data/dither/sierra-two-row-mono.png")?;
        image.clone().apply(MonoDither::SierraLite).save("data/dither/sierra-lite-mono.png")?;
        image.clone().apply(MonoDither::Bayer(2)).save("data/dither/bayer-2x2-mono.png")?;
        image.clone().apply(MonoDither::Bayer(4)).save("data/dither/bayer-4x4-mono.png")?;
        image.clone().apply(MonoDither::Bayer(8)).save("data/dither/bayer-8x8-mono.png")?;
        image.clone().apply(MonoDither::Bayer(16)).save("data/dither/bayer-16x16-mono.png")?;
        Ok(())
    }

    fn colour_websafe(image: &DynamicImage) -> ImageResult<()> {
        colour(image, palettes::WEB_SAFE.as_ref(), Some("-web-safe"))
    }
    
    fn colour_eightbit(image: &DynamicImage) -> ImageResult<()> {
        colour(image, palettes::EIGHT_BIT.as_ref(), Some("-8-bit"))
    }
    
    fn colour(image: &DynamicImage, palette: &[RgbPixel], opt_postfix: Option<&str>) -> ImageResult<()> {
        let postfix = opt_postfix.unwrap_or("");
        image.clone().apply(Dither::Basic(palette)).save(format!("data/dither/basic{}.png", postfix))?;
        image.clone().apply(Dither::FloydSteinberg(palette)).save(format!("data/dither/floyd-steinberg{}.png", postfix))?;
        image.clone().apply(Dither::JarvisJudiceNinke(palette)).save(format!("data/dither/jarvis-judice-ninke{}.png", postfix))?;
        image.clone().apply(Dither::Stucki(palette)).save(format!("data/dither/stucki{}.png", postfix))?;
        image.clone().apply(Dither::Atkinson(palette)).save(format!("data/dither/atkinson{}.png", postfix))?;
        image.clone().apply(Dither::Burkes(palette)).save(format!("data/dither/burkes{}.png", postfix))?;
        image.clone().apply(Dither::Sierra(palette)).save(format!("data/dither/sierra{}.png", postfix))?;
        image.clone().apply(Dither::SierraTwoRow(palette)).save(format!("data/dither/sierra-two-row{}.png", postfix))?;
        image.clone().apply(Dither::SierraLite(palette)).save(format!("data/dither/sierra-lite{}.png", postfix))?;
        image.clone().apply(Dither::Bayer(2, palette)).save(format!("data/dither/bayer-2x2{}.png", postfix))?;
        image.clone().apply(Dither::Bayer(4, palette)).save(format!("data/dither/bayer-4x4{}.png", postfix))?;
        image.clone().apply(Dither::Bayer(8, palette)).save(format!("data/dither/bayer-8x8{}.png", postfix))?;
        image.clone().apply(Dither::Bayer(16, palette)).save(format!("data/dither/bayer-16x16{}.png", postfix))?;
        Ok(())
    }
}