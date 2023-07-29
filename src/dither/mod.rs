use image::{Rgb, Pixel};
use num::integer::Roots;

use crate::utils::u8ops::{sq_u8, average};

pub mod algorithms;
pub mod pixel;

mod basic;
mod floydsteinberg;
mod jarvisjudiceninke;
mod stucki;
mod atkinson;
mod burkes;
mod sierra;
mod bayer;

// weighted as shown in https://shihn.ca/posts/2020/dithering/
pub fn cartesian_distance(
    (r1, g1, b1): &(u8, u8, u8),
    (r2, g2, b2): &(u8, u8, u8),
) -> u32 {
    let r_sc = ((*r2 as f32 - *r1 as f32) * 0.30).powf(2.0) as u32;
    let g_sc = ((*g2 as f32 - *g1 as f32) * 0.59).powf(2.0) as u32;
    let b_sc = ((*b2 as f32 - *b1 as f32) * 0.11).powf(2.0) as u32;

    // println!("rgb: ({}-{},{}-{},{}-{}) - scores: ({},{},{}) - distance: {}",
    //     r2, r1, g2, g1, b2, b1, r_sc, g_sc, b_sc, (r_sc + g_sc + b_sc).sqrt());

    (r_sc + g_sc + b_sc).sqrt()
}