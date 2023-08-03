use super::rgb::RgbPixel;

#[derive(Debug, Clone, Copy)]
/// Represents a pixel in the HSL colour space. Saturation and luminance are clamped at `0.0` to `1.0` - whereas hue can be any valid `f32` value.
pub struct HslPixel(f32, f32, f32);

impl From<(f32, f32, f32)> for HslPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        HslPixel(value.0, value.1, value.2)
    }
}

impl From<RgbPixel> for HslPixel {
    fn from(value: RgbPixel) -> Self {
        let (r, g, b) = value.get();
        let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

        let rgb_max = (r.max(g).max(b)) as f32;
        let rgb_min = (r.min(g).min(b)) as f32;
        let chroma = (rgb_max - rgb_min) as f32;

        let hue = if chroma == 0.0 {
            0.0
        } else if rgb_max == r {
            ((g - b) / chroma) % 6.0
        } else if rgb_max == g {
            ((b - r) / chroma) + 2.0
        } else if rgb_max == b {
            ((r - g) / chroma) + 4.0
        } else {
            panic!(
                "None of R:{} G:{} B:{} matched the RGB_MAX:{}",
                r, g, b, rgb_max
            )
        } * 60.0;

        let lightness = (rgb_max + rgb_min) / 2.0;

        let saturation = if lightness == 0.0 || lightness == 1.0 {
            0.0
        } else {
            chroma / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        return HslPixel(hue, saturation, lightness);
    }
}

impl Into<RgbPixel> for HslPixel {
    fn into(self) -> RgbPixel {
        let chroma = (1.0 - (2.0 * self.2 - 1.0).abs()) * self.1;
        let hue_degree = self.get_normalized_hue() / 60.0;
        let x = chroma * (1.0 - ((hue_degree % 2.0) - 1.0).abs());

        let hue_degree = hue_degree as i8;

        let (r1, g1, b1) = if hue_degree >= 0 && hue_degree < 1 {
            (chroma, x, 0.0)
        } else if hue_degree < 2 {
            (x, chroma, 0.0)
        } else if hue_degree < 3 {
            (0.0, chroma, x)
        } else if hue_degree < 4 {
            (0.0, x, chroma)
        } else if hue_degree < 5 {
            (x, 0.0, chroma)
        } else if hue_degree < 6 {
            (chroma, 0.0, x)
        } else {
            panic!(
                "Hue degree should be between 0 and 1 - was actually: {}",
                hue_degree
            )
        };

        let m = self.2 - (chroma / 2.0);

        RgbPixel::new(
            ((r1 + m) * 255.0).round() as u8,
            ((g1 + m) * 255.0).round() as u8,
            ((b1 + m) * 255.0).round() as u8,
        )
    }
}

impl HslPixel {
    /// Converts an HslPixel into an RgbPixel.
    pub fn to_rgb(self) -> RgbPixel {
        self.into()
    }

    /// Adds (rotates) the hue.
    pub fn add_hue(&mut self, hue: f32) -> &mut Self {
        self.0 = self.0 + hue;
        self
    }

    /// Adds saturation. Any value can be passed, but the value on the pixel is clamped to `0.0` to `1.0`.
    pub fn add_saturation(&mut self, saturation: f32) -> &mut Self {
        self.1 = (self.1 + saturation).clamp(0.0, 1.0);
        self
    }

    /// Adds luminance. Any value can be passed, but the value on the pixel is clamped to `0.0` to `1.0`.
    pub fn add_luminance(&mut self, luminance: f32) -> &mut Self {
        self.2 = (self.2 + luminance).clamp(0.0, 1.0);
        self
    }

    pub fn quantize_hue(&mut self, hues: &[f32]) -> &mut Self {
        let mut closest_dist = f32::MAX;
        let pixel_hue = self.get_normalized_hue();
        let mut current_hue = pixel_hue;

        for hue in hues.iter() {
            let normalized = Self::normalize_hue(*hue);
            let distance = (normalized - pixel_hue).abs();
            if distance < closest_dist {
                closest_dist = distance;
                current_hue = normalized;
            }
        }

        self.0 = current_hue;
        self
    }

    /// Retrieves the hue as a value between 0 and 360.
    fn get_normalized_hue(&self) -> f32 {
        Self::normalize_hue(self.0)
    }

    fn normalize_hue(hue: f32) -> f32 {
        loop {
            if hue >= 0.0 {
                break hue % 360.0;
            } else {
                break (hue % 360.0) + 360.0;
            }
        }
    }

    /// Retrieves the (h, s, l) values.
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }
}
