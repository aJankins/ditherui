use image::{Rgb, Pixel};

use crate::utils::u8ops::average;

#[derive(Debug)]
pub struct MonoPixel(u8);

#[derive(Debug)]
pub struct RgbPixel(u8, u8, u8);

pub const TWO_BIT: &'static [MonoPixel] = &[MonoPixel(0), MonoPixel(255)];

impl From<u8> for MonoPixel {
    fn from(value: u8) -> Self {
        MonoPixel(value)
    }
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

        if let (Ok(ru), Ok(gu), Ok(bu)) = (r,g,b) {
            RgbPixel(ru, gu, bu)
        } else {
            println!("WARNING! Couldn't convert {} into an RGB value. Returning black.", value);
            RgbPixel(0,0,0)
        }
    }
}

impl MonoPixel {
    pub fn mono_from(pixel: &Rgb<u8>) -> MonoPixel {
        MonoPixel(
            average(pixel.channels()) as u8
        )
    }

    pub fn add_error(self, error: i32) -> MonoPixel {
        MonoPixel((self.0 as i32 + error).min(255).max(0) as u8)
    }

    pub fn quantize(&self, palette: &[MonoPixel]) -> MonoPixel {
        let mut closest_dist = u16::MAX;
        let mut closest_col = self;

        for colour in palette.iter() {
            let distance = (colour.0 as i16 - self.0 as i16).abs() as u16;
            if distance < closest_dist {
                closest_col = colour;
                closest_dist = distance;
            }
        }

        // println!("{} ---- quantized to ---> {}", self.0, closest_col.get());

        closest_col.get().into()
    }

    pub fn get_error(&self, other: &MonoPixel) -> i32 {
        self.0 as i32 - other.0 as i32
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

impl RgbPixel {
    pub fn rgb_from(pixel: &Rgb<u8>) -> RgbPixel {
        let channels = pixel.channels();
        RgbPixel(
            channels[0],
            channels[1],
            channels[2]
        )
    }

    pub fn add_error(self, error: (i32, i32, i32)) -> RgbPixel {
        RgbPixel(
            ((self.0 as i32) + error.0).min(255).max(0) as u8,
            ((self.1 as i32) + error.1).min(255).max(0) as u8,
            ((self.2 as i32) + error.2).min(255).max(0) as u8,
        )
    }

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

    pub fn get_error(&self, other: &RgbPixel) -> (i32, i32, i32) {
        (
            self.0 as i32 - other.0 as i32,
            self.1 as i32 - other.1 as i32,
            self.2 as i32 - other.2 as i32,
        )
    }

    pub fn get_difference(&self, other: &RgbPixel) -> f64 {
        // self._redmean_diff(other)
        self._weighed_euclidean_diff(other)
        // self._weighted_cartesian_diff(other)
        // self._naive_diff(other)
    }

    fn _redmean_diff(&self, other: &RgbPixel) -> f64 {
        let avg_r = average(&[self.0, other.0]);

        let diff_r = (2.0 + avg_r / 256.0) * (self.0 as i32 - other.0 as i32).pow(2) as f64;
        let diff_g = 4 * (self.1 as i32 - other.1 as i32).pow(2);
        let diff_b = (2.0 + (255.0 - avg_r) / 256.0) * (self.0 as i32 - other.0 as i32).pow(2) as f64;

        diff_r + diff_g as f64 + diff_b
    }

    fn _weighed_euclidean_diff(&self, other: &RgbPixel) -> f64 {
        let m = 
            if self.0 > 127 {(3, 4, 2)} else {(2, 4, 3)};

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
        let r_sc = ((self.0 as f64 - other.0 as f64)).powf(2.0);
        let g_sc = ((self.1 as f64 - other.1 as f64)).powf(2.0);
        let b_sc = ((self.2 as f64 - other.2 as f64)).powf(2.0);

        r_sc + g_sc + b_sc
    }

    pub fn get(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
}