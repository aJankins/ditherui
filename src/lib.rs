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
//! ### Implementing `...Input`
//! _Because_ `[u8; 3]` is a valid `FilterInput`, anything that can be reduced to a series of
//! `RGB` values will have the actual effect logic readymade.
//! 
//! The same applies for `DitherInput`, however it's more complicated because dithering requires
//! a more complicated input. Specifically, a _2d matrix_ of RGB values (`[u8; 3]`). In other words
//! you'll need to convert your type into a _2d matrix_ first (`Vec<Vec<[u8; 3]>>`) - which already
//! implements `DitherInput`.
//! 
//! To see examples for this, check out the implementations of `FilterInput` and `DitherInput` on
//! `DynamicImage`.
pub mod dither;

/// Filters that can be applied to the image - such as brightness, contrast, and more.
pub mod filter;

/// Utilities. Mostly just for the test cases - will probably be removed.
pub mod utils;

pub mod colour;

/// Prelude for including the useful elements from the library - including algorithms, traits, and constants.
pub mod prelude {
    // algorithms
    pub use crate::dither::Dither as Dither;
    pub use crate::filter::Filter as Filter;
    
    // traits
    pub use crate::Effect;
    pub use crate::Affectable;
    pub use crate::colour::gradient::IntoGradient;

    // constants
    pub use crate::colour::colours::srgb as SrgbColour;
    pub use crate::colour::palettes;
}

pub trait EffectInput<T> {
    fn run_through(self, effect: &T) -> Self;
}

impl<T, I: EffectInput<T>> Effect<I> for T {
    fn affect(&self, item: I) -> I {
        item.run_through(self)
    }
}

/// Defines an effect that can be applied onto `T`.
/// 
/// Doing this will also implement `Affectable<E>` for `T` - where
/// `E` is the `Effect` you're implementing this on.
pub trait Effect<T> {
    fn affect(&self, item: T) -> T;
}

/// Defines something that can have an effect applied to it.
/// 
/// It doesn't necessarily need to be implemented - if `Effect<T>` is implemented on `E`,
/// then `T` will automatically implement `Affectable<E>`.
pub trait Affectable<E> {
    fn apply(self, effect: &E) -> Self;
}

impl<T, E> Affectable<E> for T where E: Effect<T> {
    fn apply(self, effect: &E) -> Self {
        effect.affect(self)
    }
}

// /// Allows the implemented struct to use `.apply` directly.
// pub trait AdjustableImage {
//     fn apply(self, effect: impl ImageEffect<Self>) -> Self;
// }

// impl AdjustableImage for DynamicImage {
//     fn apply(self, effect: impl ImageEffect<Self>) -> Self {
//         effect.apply(self)
//     }
// }

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
    use image::{DynamicImage, ImageResult};
    use palette::Srgb;

    use crate::{
        colour::{colours::srgb as RGB, gradient::IntoGradient, utils::ONE_BIT},
        prelude::{*, palettes::{WEB_SAFE, EIGHT_BIT}},
        utils::{image::load_image_from_url_with_max_dim, ImageFilterResult}, Affectable,
    };

    fn get_image() -> ImageFilterResult<DynamicImage> {
        load_image_from_url_with_max_dim("https://cdn.britannica.com/61/234061-050-6D985ED2/Carina-Nebula-Cosmic-Cliffs-NGC-3324-James-Webb-Space-Telescope-NIRCam.jpg", 1080)
    }

    #[test]
    fn dither_test() -> ImageFilterResult<()> {
        let image = get_image()?;

        let palette = [
            RGB::RED.build_gradient_lch(5),
            RGB::BLUE.build_gradient_lch(5),
            RGB::GOLD.build_gradient_lch(5),
        ].concat();

        dither(&image, &ONE_BIT, Some("-mono"))?;
        dither(&image, &WEB_SAFE, Some("-web-safe"))?;
        dither(&image, &EIGHT_BIT, Some("-8-bit"))?;
        dither(&image, &palette, Some("-custom-palette"))?;

        Ok(())
    }

    #[test]
    fn filter_effects_test() -> ImageFilterResult<()> {
        let image = get_image()?;

        image
            .clone()
            .apply(&Filter::RotateHue(180.0))
            .save("data/colour/rotate-hue-180.png")?;
        image
            .clone()
            .apply(&Filter::Brighten( 0.2))
            .save("data/colour/brighten+0.2.png")?;
        image
            .clone()
            .apply(&Filter::Brighten(-0.2))
            .save("data/colour/brighten-0.2.png")?;
        image
            .clone()
            .apply(&Filter::Saturate( 0.2))
            .save("data/colour/saturate+0.2.png")?;
        image
            .clone()
            .apply(&Filter::Saturate(-0.2))
            .save("data/colour/saturate-0.2.png")?;
        image
            .clone()
            .apply(&Filter::Contrast(0.5))
            .save("data/colour/contrast.0.5.png")?;
        image
            .clone()
            .apply(&Filter::Contrast(1.5))
            .save("data/colour/contrast.1.5.png")?;

        let gradient_map = [
            (Srgb::new(0.0, 0.0, 1.0), 0.00),
            (Srgb::new(1.0, 0.0, 0.0), 0.50),
            (Srgb::new(0.0, 1.0, 0.0), 1.00),
        ];

        image
            .clone()
            .apply(&Filter::GradientMap(&gradient_map))
            .save("data/colour/gradient-mapped.png")?;

        let hue_palette = [180.0, 300.0];

        image
            .clone()
            .apply(&Filter::QuantizeHue(&hue_palette))
            .save("data/colour/quantize-hue.png")?;

        Ok(())
    }

    fn dither(
        image: &DynamicImage,
        palette: &[Srgb],
        opt_postfix: Option<&str>,
    ) -> ImageResult<()> {
        let postfix = opt_postfix.unwrap_or("");
        image
            .clone()
            .apply(&Dither::Basic(palette))
            .save(format!("data/dither/basic{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::FloydSteinberg(palette))
            .save(format!("data/dither/floyd-steinberg{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::JarvisJudiceNinke(palette))
            .save(format!("data/dither/jarvis-judice-ninke{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::Stucki(palette))
            .save(format!("data/dither/stucki{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::Atkinson(palette))
            .save(format!("data/dither/atkinson{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::Burkes(palette))
            .save(format!("data/dither/burkes{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::Sierra(palette))
            .save(format!("data/dither/sierra{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::SierraTwoRow(palette))
            .save(format!("data/dither/sierra-two-row{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::SierraLite(palette))
            .save(format!("data/dither/sierra-lite{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::Bayer(2, palette))
            .save(format!("data/dither/bayer-2x2{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::Bayer(4, palette))
            .save(format!("data/dither/bayer-4x4{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::Bayer(8, palette))
            .save(format!("data/dither/bayer-8x8{}.png", postfix))?;
        image
            .clone()
            .apply(&Dither::Bayer(16, palette))
            .save(format!("data/dither/bayer-16x16{}.png", postfix))?;
        Ok(())
    }
}
