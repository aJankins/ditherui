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
//! - in addition to some traits.
//! 
//! # Traits and Extensibility
//! 
//! ## Effect<T> and Affectable<E>
//! 
//! Think of these as opposites - like `From<T>` and `Into<T>`. The first defines an effect that can be
//! applied on `T`, whereas the second defines something that can have an effect `E` applied to it.
//! 
//! If you'd like to extend this library with your own effect, implement `Effect<T>` on it - where `T` is
//! what it can be applied on. Once you do this, `T` will _automatically_ implement `Affectable<E>`, where
//! `E` is your effect.
//! 
//! Note that this doesn't go the opposite way - applying `Affectable<E>` on something won't implement
//! `Effect<T>` due to conflicting implementation issues. To explain this, let's assume this _did_ work...
//! 
//! - User implements `Affectable<E>` on T.
//! - `E` auto-implements `Effect<T>`...
//! - ...which then auto-implements `Affectable<E>` again.
//! 
//! In the above case, see how your implementation immediately starts conflicting with the auto-implementation.
//! This is more of a Rust limitation, but I wanted to explain the reasoning real quick incase anyone is curious
//! _(or incase there's a not-hacky workaround I don't know about)_
//! 
//! In any case - this is currently only implemented for `DynamicImage` and `[u8; 3]`. But what if you wanted to implement this
//! for another type - say a different image representation? That's where the `Input` traits come in.
//! 
//! ## `...Input`
//! 
//! Currently there are three:
//! 
//! - `dither::algorithm::MonoAlgorithmInput`
//! - `dither::algorithm::AlgorithmInput`
//! - `filter::algorithm::AlgorithmInput`
//! 
//! To support other representations, simply implement the respective `Input` trait for them. Once
//! doing so, you'll automatically gain an `Effect<T>` and `Affectable<E>` implementation. To see why,
//! here's the signature of one of these traits:
//! 
//! ```ignore
//! impl<'a, I: FilterInput> Effect<I> for Algorithms<'a>
//! ```
//! 
//! As you can see, `Effect<I>` is implemented for `Algorithms` - where `I` is _any valid input_.
//! 
//! This is mostly here as I haven't tested whether `impl Effect<YourType> for ExternalType` would
//! work, since your type is wrapped in generics. If this _does_ work then this complication could
//! be bypassed by implementing `Effect<T>` directly.
//! 
//! In any case, the `Input` option is always available to you.
//! 
//! ## Tips
//! 
//! ### Implementing `...Input`
//! _Because_ `[u8; 3]` is a valid `FilterInput`, anything that can be reduced to a series of
//! `RGB` values will have the actual effect logic readymade.
//! 
//! This _doesn't_ apply for `DitherInput` because most algorithms require various other pixels from
//! the image. I'm hoping to improve this process in some way - like extracting out the pixel selection,
//! having a good intermediate representation, or even pulling `Bayer` as a separate algorithm since _that_
//! one only requires the current pixel.

/// Contains multiple algorithms for dithering an image - both in 1-bit and RGB variants.
pub mod dither;

/// Filters that can be applied to the image - such as brightness, contrast, and more.
pub mod filter;

/// Utilities. Mostly just for the test cases - will probably be removed.
pub mod utils;

pub mod colour;

/// Prelude for including the useful elements from the library - including traits and algorithms.
pub mod prelude {
    pub use crate::dither::algorithms::Algorithms as Dither;
    pub use crate::dither::algorithms::MonoAlgorithms as MonoDither;
    pub use crate::filter::algorithms::Algorithms as Filter;
    // pub use crate::AdjustableImage;
    pub use crate::Effect;

    pub use crate::colour::colours::srgb as SrgbColour;
    pub use crate::colour::gradient::IntoGradient;
    pub use crate::dither::error;
    pub use crate::colour::palettes;
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
/// Helps construct a gradient map from HSL values.
///
/// You *could* construct the map yourself, however the purpose of this is mostly to
/// provide an easily usable and *clean* way to generate a gradient map from HSL values.
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
        colour::{colours::srgb as RGB, gradient::IntoGradient, palettes},
        prelude::*,
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

        mono(&image)?;
        colour_websafe(&image)?; // takes a long time due to large palette
        colour_eightbit(&image)?; // significantly faster
        colour(&image, &palette, Some("-custom-palette"))?;

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

    fn mono(image: &DynamicImage) -> ImageResult<()> {
        image
            .clone()
            .apply(&MonoDither::Basic)
            .save("data/dither/basic-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::FloydSteinberg)
            .save("data/dither/floyd-steinberg-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::JarvisJudiceNinke)
            .save("data/dither/jarvis-judice-ninke-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::Stucki)
            .save("data/dither/stucki-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::Atkinson)
            .save("data/dither/atkinson-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::Burkes)
            .save("data/dither/burkes-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::Sierra)
            .save("data/dither/sierra-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::SierraTwoRow)
            .save("data/dither/sierra-two-row-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::SierraLite)
            .save("data/dither/sierra-lite-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::Bayer(2))
            .save("data/dither/bayer-2x2-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::Bayer(4))
            .save("data/dither/bayer-4x4-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::Bayer(8))
            .save("data/dither/bayer-8x8-mono.png")?;
        image
            .clone()
            .apply(&MonoDither::Bayer(16))
            .save("data/dither/bayer-16x16-mono.png")?;
        Ok(())
    }

    fn colour_websafe(image: &DynamicImage) -> ImageResult<()> {
        colour(image, palettes::WEB_SAFE.as_ref(), Some("-web-safe"))
    }

    fn colour_eightbit(image: &DynamicImage) -> ImageResult<()> {
        colour(image, palettes::EIGHT_BIT.as_ref(), Some("-8-bit"))
    }

    fn colour(
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
