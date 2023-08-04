/// Monochromatic pixels. Effectively represent RGB pixels, except each channel is equivalent.
pub mod mono;

/// RGB pixels. Have 3 components for Red, Green, and Blue.
pub mod rgb;

/// HSL pixels. Have 3 components for Hue, Saturation, and Luminance.
pub mod hsl;

/// LAB pixels. Have 3 components for Luma, a, and b.
pub mod lab;

/// LCH pixels. Have 3 components for Luma, Chroma, and Hue.
pub mod lch;

pub mod oklab;

pub mod oklch;

/// The raw conversion algorithms between multiple colour schemes. 
/// 
/// Implementation is inspired by `color.js` - especially the [spaces](https://github.com/LeaVerou/color.js/tree/main/src/spaces)
/// folder. Since it's raw, some conversions (rgb to lch) don't have a specific function for them.
/// 
/// To convert, you should use the utilities in each color space module - especially since some conversions require intermediary steps.
/// As an example, here are all the steps to convert RGB to LCH:
/// 
/// ```text
/// RGB -> XYZ_D65 -> XYZ_D50 -> LAB -> LCH
/// ```
/// 
/// Instead, you could just use the `.as_lch()` method on an `RgbPixel` to do this for you.
pub mod conversions;

pub mod comparisons;