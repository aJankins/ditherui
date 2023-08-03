use image::Rgb;

use crate::utils::numops::average;

use super::hsl::HslPixel;

#[derive(Debug, Clone, Copy)]
/// Represents a pixel in the RGB colour space. Each value (RGB) ranges between 0 and 255.
pub struct RgbPixel(u8, u8, u8);

pub mod colours {
    use super::RgbPixel;

    // 1-bit
    pub static BLACK: RgbPixel = RgbPixel(0, 0, 0);
    pub static WHITE: RgbPixel = RgbPixel(255, 255, 255);

    // primary colours
    pub static RED: RgbPixel = RgbPixel(255, 0, 0);
    pub static GREEN: RgbPixel = RgbPixel(0, 255, 0);
    pub static BLUE: RgbPixel = RgbPixel(0, 0, 255);

    // secondary colours
    pub static YELLOW: RgbPixel = RgbPixel(255, 255, 0);
    pub static PURPLE: RgbPixel = RgbPixel(255, 0, 255);
    pub static CYAN: RgbPixel = RgbPixel(0, 255, 255);

    // other
    pub static PINK: RgbPixel = RgbPixel(255, 150, 200);
    pub static MAGENTA: RgbPixel = RgbPixel(255, 40, 200);
    pub static ROSE: RgbPixel = RgbPixel(255, 0, 150);

    pub static GOLD: RgbPixel = RgbPixel(255, 200, 40);
    pub static ORANGE: RgbPixel = RgbPixel(255, 100, 0);
    pub static RUST: RgbPixel = RgbPixel(180, 50, 0);

    pub static AQUAMARINE: RgbPixel = RgbPixel(0, 255, 150);
}

impl From<(u8, u8, u8)> for RgbPixel {
    fn from(value: (u8, u8, u8)) -> Self {
        RgbPixel(value.0, value.1, value.2)
    }
}

impl From<&str> for RgbPixel {
    fn from(value: &str) -> Self {
        let r = u8::from_str_radix(&value[0..=1], 16);
        let g = u8::from_str_radix(&value[2..=3], 16);
        let b = u8::from_str_radix(&value[4..=5], 16);

        if let (Ok(ru), Ok(gu), Ok(bu)) = (r, g, b) {
            RgbPixel(ru, gu, bu)
        } else {
            println!(
                "WARNING! Couldn't convert {} into an RGB value. Returning black.",
                value
            );
            RgbPixel(0, 0, 0)
        }
    }
}

impl From<&Rgb<u8>> for RgbPixel {
    fn from(value: &Rgb<u8>) -> Self {
        let [r, g, b] = value.0;
        RgbPixel(r, g, b)
    }
}

impl RgbPixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RgbPixel(r, g, b)
    }

    /// Adds an error to each of the channels.
    pub fn add_error(self, error: (i32, i32, i32)) -> RgbPixel {
        RgbPixel(
            ((self.0 as i32) + error.0).min(255).max(0) as u8,
            ((self.1 as i32) + error.1).min(255).max(0) as u8,
            ((self.2 as i32) + error.2).min(255).max(0) as u8,
        )
    }

    /// Quantizes the RGB pixel to the nearest colour in the palette.
    pub fn quantize(&self, palette: &[RgbPixel]) -> RgbPixel {
        let mut closest_distance = f64::MAX;
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
        let mix_calc = |pixchan1: u8, pixchan2: u8| {
            (pixchan1 as f32 * ratio) + (pixchan2 as f32) * (1.0 - ratio)
        };
        RgbPixel(
            mix_calc(self.0, other.0).round() as u8,
            mix_calc(self.1, other.1).round() as u8,
            mix_calc(self.2, other.2).round() as u8,
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
                self.to_hsl()
                    // set the luminance to black first
                    .add_luminance(-2.0)
                    .add_luminance(i as f32 * fractional)
                    .to_rgb()
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
    pub fn get_error(&self, other: &RgbPixel) -> (i32, i32, i32) {
        (
            self.0 as i32 - other.0 as i32,
            self.1 as i32 - other.1 as i32,
            self.2 as i32 - other.2 as i32,
        )
    }

    /// Retrieves the difference between it and another `RgbPixel` using the
    /// weighted euclidean method.
    pub fn get_difference(&self, other: &RgbPixel) -> f64 {
        self._weighed_euclidean_diff(other)
    }

    fn _redmean_diff(&self, other: &RgbPixel) -> f64 {
        let avg_r = average(&[self.0, other.0]);

        let diff_r = (2.0 + avg_r / 256.0) * (self.0 as i32 - other.0 as i32).pow(2) as f64;
        let diff_g = 4 * (self.1 as i32 - other.1 as i32).pow(2);
        let diff_b =
            (2.0 + (255.0 - avg_r) / 256.0) * (self.0 as i32 - other.0 as i32).pow(2) as f64;

        diff_r + diff_g as f64 + diff_b
    }

    fn _weighed_euclidean_diff(&self, other: &RgbPixel) -> f64 {
        let m = if self.0 > 127 { (3, 4, 2) } else { (2, 4, 3) };

        let diff_r = m.0 as f64 * (self.0 as f64 - other.0 as f64).powf(2.0);
        let diff_g = m.1 as f64 * (self.1 as f64 - other.1 as f64).powf(2.0);
        let diff_b = m.2 as f64 * (self.2 as f64 - other.2 as f64).powf(2.0);

        diff_r + diff_g + diff_b
    }

    fn _weighted_cartesian_diff(&self, other: &RgbPixel) -> f64 {
        let r_sc = ((self.0 as f64 - other.0 as f64) * 0.30).powf(2.0);
        let g_sc = ((self.1 as f64 - other.1 as f64) * 0.59).powf(2.0);
        let b_sc = ((self.2 as f64 - other.2 as f64) * 0.11).powf(2.0);

        r_sc + g_sc + b_sc
    }

    fn _naive_diff(&self, other: &RgbPixel) -> f64 {
        let r_sc = (self.0 as f64 - other.0 as f64).powf(2.0);
        let g_sc = (self.1 as f64 - other.1 as f64).powf(2.0);
        let b_sc = (self.2 as f64 - other.2 as f64).powf(2.0);

        r_sc + g_sc + b_sc
    }

    /// Retrieves the (r, g, b) channels of the pixel.
    pub fn get(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    /// Converts the pixel to an `HslPixel`.
    pub fn to_hsl(self) -> HslPixel {
        self.into()
    }
}
