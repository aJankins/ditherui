use image::{Rgb, Pixel};
use num::integer::Roots;

use crate::utils::u8ops::sq_u8;

pub mod algorithms;

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
    let r_sc = (*r2 as i32 - *r1 as i32).pow(2) as u32;
    let g_sc = (*g2 as i32 - *g1 as i32).pow(2) as u32;
    let b_sc = (*b2 as i32 - *b1 as i32).pow(2) as u32;

    (r_sc + g_sc + b_sc).sqrt()
}

pub fn get_closest_color_to(
    original: (u8, u8, u8),
    palette: &[(u8, u8, u8)]
) -> &(u8, u8, u8) {
    let mut closest_distance = 999999999;
    let mut current_colour = &(0,0,0);

    for colour in palette.iter() {
        let distance = cartesian_distance(colour, &original);
        if distance < closest_distance {
            current_colour = colour;
            closest_distance = distance;
        };
    }

    current_colour
}

pub fn truple_from_rgb(rgb: &Rgb<u8>) -> (u8, u8, u8) {
    let channels = rgb.channels();
    (
        channels[0], channels[1], channels[2],
    )
}