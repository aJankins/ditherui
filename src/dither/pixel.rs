use image::{Rgb, Pixel};

use crate::utils::u8ops::average;

use super::cartesian_distance;

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
        let mut closest_distance = u32::MAX;
        let mut current_colour = self;
    
        for colour in palette.iter() {
            let distance = cartesian_distance(&colour.get(), &self.get());
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

    pub fn get(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
}