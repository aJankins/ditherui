//! This crate provides multiple effects that can be applied on an image.
//!
//! Currently there's two classes of effects:
//!
//! 1. [**Dithering**](./dither/algorithms/enum.Dither.html) - Limiting the colour palette of a given image while still
//!    retaining more detail than a purely quantized approach would.
//! 2. [**Filtering**](./filter/algorithms/enum.Filter.html) - Some more common effects applied on an image, such as brightness,
//!    contrast, gradient mapping, and more.
//!
//! This crate assumes that you are using the `image` crate for processing - as all these
//! algorithms work specifically with the `DynamicImage` struct (for now).
//!
//! The *prelude* is useful for importing some common functionality, like the algorithms themselves
//! alongside some traits.
//! 
//! # Traits and Extensibility
//! 
//! ## `Effect<T>` and `Affectable<E>`
//! 
//! Think of these as opposites - like `From<T>` and `Into<T>`. The first defines an effect that can be
//! applied on `T`, whereas the second defines something that can have an effect `E` applied to it.
//! 
//! If you'd like to extend this library with your own effect, implement [`Effect<T>`](./trait.Effect.html) on it - where `T` is
//! what it can be applied on. Once you do this, `T` will _automatically_ implement [`Affectable<E>`](./trait.Affectable.html), where
//! `E` is your effect.
//! 
//! Note that this doesn't go the opposite way - applying `Affectable<E>` on something won't implement
//! `Effect<T>` due to conflicting implementation issues. However, if you _would_ like to define a new type
//! to apply an effect on, that's where [`EffectInput<T>`](./trait.EffectInput.html) comes in.
//! 
//! ## `EffectInput<T>`
//! 
//! This trait defines an _input_ to an effect. For example, implementing `EffectInput<Filter<'a>>` on T means
//! that `Filter<'a>` accepts `T` as input. As a result, you also get the following implementations for free:
//! 
//! - `impl Effect<T> for Filter<'a>`
//! - `impl Affectable<Filter<'a>> for T`
//! 
//! As you can see, `EffectInput<T>` is almost identical to `Affectable<T>`. The only difference being that you can 
//! _actually_ implement it without conflicting implementations.
//! 
//! So for example, if you'd like to support _dithering_ for `MyType`, you can do this:
//! 
//! ```ignore
//! impl<'a, I: EffectInput<Dither<'a>> for MyType { /* ... */ }
//! ```
//! 
//! ...at which point you can call `.apply` on an instance of `MyType` and pass in a `Dither`.
//! 
//! ## Summary
//! - If you want to add an effect `E` which works with an image `I`, implement `Effect<I>` for `E`.
//! - If you want to add an image `I` that works with an effect `E`, implement `EffectInput<E>` for `I`.
//!   - ...which will automatically implement `Effect<E>` for `I` as well.
//! 
//! ## Tips
//! 
//! ### Implementing `EffectInput` for existing effects
//! _Because_ `[u8; 3]` is a valid `EffectInput<Filter>`, anything that can be reduced to a series of
//! `RGB` values will have the actual effect logic readymade.
//! 
//! The same applies for `EffectInput<Dither>`, however it's more complicated because dithering requires
//! a more complicated input. Specifically, a _2d matrix_ of RGB values (`[u8; 3]`). In other words
//! you'll need to convert your type into a _2d matrix_ first (`Vec<Vec<[u8; 3]>>`) - which already
//! implements `EffectInput<Dither>`.
//! 
//! To see examples for this, check out the implementations of `EffectInput<Dither>` and `EffectInput<Filter>` on
//! `DynamicImage`.



pub mod dither;

/// Filters that can be applied to the image - such as brightness, contrast, and more.
pub mod filter;

/// Utilities. Mostly just for the test cases - will probably be removed.
pub mod utils;

/// Colour related logic, such as distance functions, palettes, gradient generation, etc.
pub mod colour;

pub mod effect;

/// Prelude for including the useful elements from the library - including algorithms, traits, and constants.
pub mod prelude {
    // algorithms
    pub use crate::dither;
    pub use crate::filter::filters;
    
    // traits
    pub use crate::effect::Effect;
    pub use crate::effect::Affectable;
    pub use crate::colour::gradient::{
        IntoGradient,
        IntoGradientHsl,
        IntoGradientLch,
        IntoGradientOklch,
    };

    // constants
    pub use crate::colour::colours::srgb as SrgbColour;
    pub use crate::colour::palettes;
}

#[macro_export]
/// Helps construct a gradient map from colours.
///
/// You *could* construct the map yourself, however the purpose of this is mostly to
/// provide an easily usable and *clean* way to construct a gradient map.
///
/// The following is an example usage of this macro:
/// ```ignore
/// let hsl: GradientMap<Hsl<Srgb>> = gradient_map!(
///     0.00 => Hsl::new(0.0, 0.0, 0.0),
///     1.00 => Hsl::new(0.0, 0.0, 1.0),
/// );
/// ```
macro_rules! gradient_map {
    [$($threshold:expr => $color:expr),*] => {
        &[
            $(
                ($color, $threshold)
            ),*
        ]
    };
}

pub type GradientMap<'a, Color> = &'a [(Color, f32)];


#[cfg(test)]
mod test {
    use std::error::Error;

    use image::{DynamicImage, ImageResult, GenericImageView, imageops};
    use palette::{Srgb, named};

    use crate::{
        colour::{utils::ONE_BIT},
        prelude::{*, palettes::{WEB_SAFE, EIGHT_BIT}}, dither::{FLOYD_STEINBERG, JARVIS_JUDICE_NINKE, STUCKI, ATKINSON, BURKES, SIERRA, SIERRA_TWO_ROW, SIERRA_LITE, bayer::Bayer},
    };

    type UtilResult<T> = Result<T,Box<dyn Error>>;

    const IMAGE_URL: &'static str = "https://clipart-library.com/image_gallery/n781743.png";
    const MAX_DIM: Option<usize> = Some(500);

    fn get_image() -> UtilResult<DynamicImage> {
        let img_bytes = reqwest::blocking::get(IMAGE_URL)?.bytes()?;
        let image = image::load_from_memory(&img_bytes)?;
        
        let image = if let Some(max_dim) = MAX_DIM {
            let (x, y) = image.dimensions();
            if max_dim < x.max(y) as usize {
                let image = &image;let factor = max_dim as f32 / x.max(y) as f32;
                let mul = |int: u32, float: f32| (int as f32 * float) as u32;
                image.resize(mul(x, factor), mul(y, factor), imageops::Nearest)
            } else { image }
        } else { image };

        Ok(image)
    }

    #[test]
    fn dither_test() -> UtilResult<()> {
        let image = get_image()?;

        let palette = [
            named::PURPLE.into_format().build_gradient_lch(10),
            named::GOLD.into_format().build_gradient_lch(10),
        ].concat();

        dither(&image, ONE_BIT.to_vec(), Some("-mono"))?;
        dither(&image, WEB_SAFE.to_vec(), Some("-web-safe"))?;
        dither(&image, EIGHT_BIT.to_vec(), Some("-8-bit"))?;
        dither(&image, palette, Some("-custom-palette"))?;

        Ok(())
    }

    #[test]
    fn filter_effects_test() -> UtilResult<()> {
        let image = get_image()?;

        image
            .clone()
            .apply(&filters::HueRotate(180.0))
            .save("data/colour/rotate-hue-180.png")?;
        image
            .clone()
            .apply(&filters::Brighten( 0.2))
            .save("data/colour/brighten+0.2.png")?;
        image
            .clone()
            .apply(&filters::Brighten(-0.2))
            .save("data/colour/brighten-0.2.png")?;
        image
            .clone()
            .apply(&filters::Saturate( 0.2))
            .save("data/colour/saturate+0.2.png")?;
        image
            .clone()
            .apply(&filters::Saturate(-0.2))
            .save("data/colour/saturate-0.2.png")?;
        image
            .clone()
            .apply(&filters::Contrast(0.5))
            .save("data/colour/contrast.0.5.png")?;
        image
            .clone()
            .apply(&filters::Contrast(1.5))
            .save("data/colour/contrast.1.5.png")?;

        let _gradient_map = [
            (Srgb::new(0.0, 0.0, 1.0), 0.00),
            (Srgb::new(1.0, 0.0, 0.0), 0.50),
            (Srgb::new(0.0, 1.0, 0.0), 1.00),
        ];

        let mut gradient_map = filters::GradientMap::new();
        gradient_map
            .add_entry(Srgb::new(0.0, 0.0, 1.0), 0.00)
            .add_entry(Srgb::new(1.0, 0.0, 0.0), 0.50)
            .add_entry(Srgb::new(0.0, 1.0, 0.0), 1.00);

        image
            .clone()
            .apply(&gradient_map)
            .save("data/colour/gradient-mapped.png")?;

        let hue_palette = vec![180.0, 300.0];

        image
            .clone()
            .apply(&filters::QuantizeHue::with_hues(hue_palette))
            .save("data/colour/quantize-hue.png")?;

        image
            .clone()
            .apply(&filters::MultiplyHue(4.0))
            .save("data/colour/multiply-hue.4.0.png")?;

        image
            .clone()
            .apply(&filters::MultiplyHue(12.0))
            .save("data/colour/multiply-hue.12.0.png")?;

        Ok(())
    }

    fn dither(
        image: &DynamicImage,
        palette: Vec<Srgb>,
        opt_postfix: Option<&str>,
    ) -> ImageResult<()> {
        let postfix = opt_postfix.unwrap_or("");

        let error_propagators = vec![
            FLOYD_STEINBERG,
            JARVIS_JUDICE_NINKE,
            STUCKI,
            ATKINSON,
            BURKES,
            SIERRA,
            SIERRA_TWO_ROW,
            SIERRA_LITE
        ];

        for propagator in error_propagators.into_iter() {
            image.clone()
                .apply(&propagator.with_palette(palette.clone()))
                .save(format!("data/dither/{}{}.png", propagator.name, postfix))?;
        }

        image.clone().apply(&Bayer::new(2, palette.clone()))
            .save(format!("data/dither/bayer-2x2{}.png", postfix))?;
        image.clone().apply(&Bayer::new(4, palette.clone()))
            .save(format!("data/dither/bayer-4x4{}.png", postfix))?;
        image.clone().apply(&Bayer::new(8, palette.clone()))
            .save(format!("data/dither/bayer-8x8{}.png", postfix))?;
        image.clone().apply(&Bayer::new(16, palette.clone()))
            .save(format!("data/dither/bayer-16x16{}.png", postfix))?;
        Ok(())
    }
}
