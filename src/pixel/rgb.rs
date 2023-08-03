use image::Rgb;

use crate::utils::numops::average;

use super::{hsl::HslPixel, lab::LabPixel, conversions::{rgb_to_hsl, chain_conversions, rgb_to_xyz_d65, xyz_d65_to_xyz_d50}, lch::LchPixel};

#[derive(Debug, Clone, Copy)]
/// Represents a pixel in the RGB colour space. Each value (RGB) ranges between 0 and 255.
pub struct RgbPixel(pub f32, pub f32, pub f32);

pub mod colours {
    use super::RgbPixel;

    // 1-bit
    pub static BLACK: RgbPixel = RgbPixel(0.0, 0.0, 0.0);
    pub static WHITE: RgbPixel = RgbPixel(255.0, 255.0, 255.0);

    // primary colours
    pub static RED: RgbPixel = RgbPixel(255.0, 0.0, 0.0);
    pub static GREEN: RgbPixel = RgbPixel(0.0, 255.0, 0.0);
    pub static BLUE: RgbPixel = RgbPixel(0.0, 0.0, 255.0);

    // secondary colours
    pub static YELLOW: RgbPixel = RgbPixel(255.0, 255.0, 0.0);
    pub static PURPLE: RgbPixel = RgbPixel(255.0, 0.0, 255.0);
    pub static CYAN: RgbPixel = RgbPixel(0.0, 255.0, 255.0);

    // other
    pub static PINK: RgbPixel = RgbPixel(255.0, 150.0, 200.0);
    pub static MAGENTA: RgbPixel = RgbPixel(255.0, 40.0, 200.0);
    pub static ROSE: RgbPixel = RgbPixel(255.0, 0.0, 150.0);

    pub static GOLD: RgbPixel = RgbPixel(255.0, 200.0, 40.0);
    pub static ORANGE: RgbPixel = RgbPixel(255.0, 100.0, 0.0);
    pub static RUST: RgbPixel = RgbPixel(180.0, 50.0, 0.0);

    pub static AQUAMARINE: RgbPixel = RgbPixel(0.0, 255.0, 150.0);
}

impl From<(u8, u8, u8)> for RgbPixel {
    fn from(value: (u8, u8, u8)) -> Self {
        RgbPixel(
            value.0 as f32 / 255.0,
            value.1 as f32 / 255.0,
            value.2 as f32 / 255.0,
        )
    }
}

impl From<(f32, f32, f32)> for RgbPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        RgbPixel(value.0, value.1, value.2)
    }
}

impl From<&str> for RgbPixel {
    fn from(value: &str) -> Self {
        let r = u8::from_str_radix(&value[0..=1], 16);
        let g = u8::from_str_radix(&value[2..=3], 16);
        let b = u8::from_str_radix(&value[4..=5], 16);

        if let (Ok(ru), Ok(gu), Ok(bu)) = (r, g, b) {
            RgbPixel(
                ru as f32 / 255.0,
                gu as f32 / 255.0,
                bu as f32 / 255.0,
            )
        } else {
            println!(
                "WARNING! Couldn't convert {} into an RGB value. Returning black.",
                value
            );
            RgbPixel(0.0, 0.0, 0.0)
        }
    }
}

impl From<&Rgb<u8>> for RgbPixel {
    fn from(value: &Rgb<u8>) -> Self {
        let [r, g, b] = value.0;
        RgbPixel(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }
}

impl RgbPixel {
    /// Adds an error to each of the channels.
    pub fn add_error(self, error: (f32, f32, f32)) -> RgbPixel {
        RgbPixel(
            (self.0 + error.0).min(1.0).max(0.0),
            (self.1 + error.1).min(1.0).max(0.0),
            (self.2 + error.2).min(1.0).max(0.0),
        )
    }

    /// Quantizes the RGB pixel to the nearest colour in the palette.
    pub fn quantize(&self, palette: &[RgbPixel]) -> RgbPixel {
        let mut closest_distance = f32::MAX;
        let mut current_colour = self;

        for colour in palette.iter() {
            let distance = colour.get_difference(self);
            if distance < closest_distance {
                current_colour = colour;
                closest_distance = distance;
            };
        }

        current_colour.get().into()
    }

    /// Mixes two colours together to produce a third colour.
    ///
    /// Takes a factor that determines how much priority to give the *current* pixel.
    ///
    /// - `0.5` mixes equally.
    /// - `> 0.5` prioritizes the *current* pixel.
    /// - `< 0.5` prioritizes the *other/parameter* pixel.
    ///
    /// Putting it another way:
    ///
    /// ```ignore
    /// RED.mix(&BLUE, 0.0) = BLUE
    /// RED.mix(&BLUE, 1.0) = RED
    /// ```
    pub fn mix(&self, ratio: f32, other: &RgbPixel) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let mix_calc = |pixchan1: f32, pixchan2: f32| {
            (pixchan1 * ratio) + pixchan2 * (1.0 - ratio)
        };
        RgbPixel(
            mix_calc(self.0, other.0),
            mix_calc(self.1, other.1),
            mix_calc(self.2, other.2),
        )
    }

    /// This function will build a gradient out of the current colour.
    /// by generating a list of said colour with varying luminance - utilising HSL.
    ///
    /// `shades` determines how many shades get generated. Passing `1` will
    /// return a vector with a single colour containing `0.5` luminance - for example.
    ///
    /// **Note:** This will *not* include black and white.
    pub fn build_gradient(&self, shades: u16) -> Vec<Self> {
        let fractional = 1 as f32 / (shades + 1) as f32;
        (1..=shades)
            .into_iter()
            .map(|i| {
                self.as_hsl()
                    // set the luminance to black first
                    .add_luminance(-2.0)
                    .add_luminance(i as f32 * fractional)
                    .as_rgb()
            })
            .collect()
    }

    /// This function will build a gradient by mixing the current colour with another
    /// using various ratios.
    ///
    /// `mixes` determines how many mixes get generated. Passing `1` will return
    /// a vector with a single colour that mixes both equally.
    ///
    /// **Note:** This will *not* include either **pure** colour - only mixes.
    pub fn build_gradient_mix(&self, other: &RgbPixel, mixes: u16) -> Vec<Self> {
        let fractional = 1 as f32 / (mixes + 1) as f32;
        (1..=mixes)
            .into_iter()
            .map(|i| self.mix(i as f32 * fractional, other))
            .collect()
    }

    /// Gets the error in channel values between itself and another `RgbPixel`.
    pub fn get_error(&self, other: &RgbPixel) -> (f32, f32, f32) {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
    }

    /// Retrieves the difference between it and another `RgbPixel` using the
    /// weighted euclidean method.
    pub fn get_difference(&self, other: &RgbPixel) -> f32 {
        let m = if self.0 > 0.5 { (3.0, 4.0, 2.0) } else { (2.0, 4.0, 3.0) };

        let diff_r = m.0 * (self.0 - other.0).powi(2);
        let diff_g = m.1 * (self.1 - other.1).powi(2);
        let diff_b = m.2 * (self.2 - other.2).powi(2);

        diff_r + diff_g + diff_b
    }

    /// Retrieves the (r, g, b) channels of the pixel as a tuple.
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn get_u8(&self) -> (u8, u8, u8) {
        (
            (self.0 * 255.0).round() as u8,
            (self.1 * 255.0).round() as u8,
            (self.2 * 255.0).round() as u8,
        )
    }

    /// Converts the pixel to an `HslPixel`.
    pub fn as_hsl(&self) -> HslPixel {
        HslPixel::from_rgb(self)
    }

    /// Converts the pixel to a `LabPixel`.
    pub fn as_lab(&self) -> LabPixel {
        LabPixel::from_rgb(self)
    }

    /// Converts the pixel to a `LchPixel`.
    pub fn as_lch(&self) -> LchPixel {
        LchPixel::from_rgb(self)
    }
}
