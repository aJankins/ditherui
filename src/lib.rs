//! This crate provides functionality

use image::DynamicImage;

/// dither
pub mod dither;

/// colour
pub mod colour;

/// utils
pub mod utils;

pub trait ImageEffect<T: ?Sized> {
    fn apply(&self, image: T) -> T;
}

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
        utils::image::load_image,
        colour::{pixel::{rgb::RgbPixel, hsl::HslPixel}, palettes},
        AdjustableImage
    };

    use super::dither::algorithms::Algorithms as Dithers;
    use super::colour::algorithms::Algorithms as Colours;

    #[test]
    fn _exec_test() -> ImageResult<()> {
        let image = load_image("data/input.png");

        let gradient = [
            (HslPixel::from((   0.0, 0.0, 0.0 )).to_rgb(), 0.00),
            (HslPixel::from(( 325.0, 0.5, 0.4 )).to_rgb(), 0.40),
            (HslPixel::from(( 300.0, 0.5, 0.5 )).to_rgb(), 0.50),
            (HslPixel::from(( 250.0, 0.5, 0.6 )).to_rgb(), 0.60),
            (HslPixel::from(( 200.0, 0.5, 0.7 )).to_rgb(), 0.70),
            (HslPixel::from(( 400.0, 0.5, 0.8 )).to_rgb(), 0.80),
            (HslPixel::from((   0.0, 0.0, 1.0 )).to_rgb(), 1.00),
        ];

        let palette = [
            "FFFFFF",
            "003355", "0088AA", "00FFDD",
            "660055", "BB00AA", "FF00EE",
            "FFEE44",
            "000000",
        ].map(|hex| hex.into());

        image
            // .apply(Colours::GradientMap(&gradient))
            .apply(Colours::Brighten(0.1))
            .apply(Colours::Contrast(1.3))
            .apply(Dithers::Bayer(8, &palette))
            .save("data/output.png")?;

        Ok(())
    }

    // #[test]
    fn dither_test() -> ImageResult<()> {    
        let image = load_image("data/input.png");
    
        let palette: &[RgbPixel] = &[
            "FFFFFF",
            "003355", "0088AA", "00FFDD",
            "660055", "BB00AA", "FF00EE",
            // "FFEE44",
            "000000",
        ].map(|tuple| tuple.into());
    
        // let palette: &[RgbPixel] = &[
        //     "FFFFFF",
        //     "440055", "9900AA", "EE00FF", "FF00FF",
        //     "551100", "AA5500", "FFAA00", "FFFF00",
        //     "000000",
        // ].map(|tuple| tuple.into());

        
        mono(&image)?;
        colour_websafe(&image)?;                              // takes a long time due to large palette
        colour_eightbit(&image)?;                             // significantly faster
        colour(&image, palette, Some("-custom-palette"))?;    // custom palettes, uncomment a palette above for examples
    
        Ok(())
    }

    // #[test]
    fn colour_effects_test() -> ImageResult<()> {
        let image = load_image("data/original.png");

        image.clone().apply(Colours::RotateHue(180.0)).save("data/colour/rotate-hue-180.png")?;
        image.clone().apply(Colours::Brighten( 0.2)).save("data/colour/brighten+0.2.png")?;
        image.clone().apply(Colours::Brighten(-0.2)).save("data/colour/brighten-0.2.png")?;
        image.clone().apply(Colours::Saturate( 0.2)).save("data/colour/saturate+0.2.png")?;
        image.clone().apply(Colours::Saturate(-0.2)).save("data/colour/saturate-0.2.png")?;
        image.clone().apply(Colours::Contrast( 0.5)).save("data/colour/contrast.0.5.png")?;
        image.clone().apply(Colours::Contrast( 1.5)).save("data/colour/contrast.1.5.png")?;

        let gradient_map = [
            ("000000".into(), 0.00),
            ("0000FF".into(), 0.25),
            ("FF0000".into(), 0.50),
            ("00FF00".into(), 0.75),
            ("FFFFFF".into(), 1.00),
        ];

        image.clone().apply(Colours::GradientMap(&gradient_map)).save("data/colour/gradient-mapped.png")?;

        Ok(())
    }

    fn mono(image: &DynamicImage) -> ImageResult<()> {
        image.clone().apply(Dithers::BasicMono).save("data/dither/basic-mono.png")?;
        image.clone().apply(Dithers::FloydSteinbergMono).save("data/dither/floyd-steinberg-mono.png")?;
        image.clone().apply(Dithers::JarvisJudiceNinkeMono).save("data/dither/jarvis-judice-ninke-mono.png")?;
        image.clone().apply(Dithers::StuckiMono).save("data/dither/stucki-mono.png")?;
        image.clone().apply(Dithers::AtkinsonMono).save("data/dither/atkinson-mono.png")?;
        image.clone().apply(Dithers::BurkesMono).save("data/dither/burkes-mono.png")?;
        image.clone().apply(Dithers::SierraMono).save("data/dither/sierra-mono.png")?;
        image.clone().apply(Dithers::SierraTwoRowMono).save("data/dither/sierra-two-row-mono.png")?;
        image.clone().apply(Dithers::SierraLiteMono).save("data/dither/sierra-lite-mono.png")?;
        image.clone().apply(Dithers::BayerMono(2)).save("data/dither/bayer-2x2-mono.png")?;
        image.clone().apply(Dithers::BayerMono(4)).save("data/dither/bayer-4x4-mono.png")?;
        image.clone().apply(Dithers::BayerMono(8)).save("data/dither/bayer-8x8-mono.png")?;
        image.clone().apply(Dithers::BayerMono(16)).save("data/dither/bayer-16x16-mono.png")?;
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
        image.clone().apply(Dithers::Basic(palette)).save(format!("data/dither/basic{}.png", postfix))?;
        image.clone().apply(Dithers::FloydSteinberg(palette)).save(format!("data/dither/floyd-steinberg{}.png", postfix))?;
        image.clone().apply(Dithers::JarvisJudiceNinke(palette)).save(format!("data/dither/jarvis-judice-ninke{}.png", postfix))?;
        image.clone().apply(Dithers::Stucki(palette)).save(format!("data/dither/stucki{}.png", postfix))?;
        image.clone().apply(Dithers::Atkinson(palette)).save(format!("data/dither/atkinson{}.png", postfix))?;
        image.clone().apply(Dithers::Burkes(palette)).save(format!("data/dither/burkes{}.png", postfix))?;
        image.clone().apply(Dithers::Sierra(palette)).save(format!("data/dither/sierra{}.png", postfix))?;
        image.clone().apply(Dithers::SierraTwoRow(palette)).save(format!("data/dither/sierra-two-row{}.png", postfix))?;
        image.clone().apply(Dithers::SierraLite(palette)).save(format!("data/dither/sierra-lite{}.png", postfix))?;
        image.clone().apply(Dithers::Bayer(2, palette)).save(format!("data/dither/bayer-2x2{}.png", postfix))?;
        image.clone().apply(Dithers::Bayer(4, palette)).save(format!("data/dither/bayer-4x4{}.png", postfix))?;
        image.clone().apply(Dithers::Bayer(8, palette)).save(format!("data/dither/bayer-8x8{}.png", postfix))?;
        image.clone().apply(Dithers::Bayer(16, palette)).save(format!("data/dither/bayer-16x16{}.png", postfix))?;
        Ok(())
    }
}