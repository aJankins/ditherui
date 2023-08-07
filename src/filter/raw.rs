use palette::{Srgb, FromColor, Lch, SetHue, Lighten, Darken, ShiftHue};

use crate::colour::utils;

// consts
pub const CHROMA_BOUND: f32 = 128.0;

// utils
#[inline] pub fn rgb_to_srgb(rgb: [u8; 3]) -> [f32; 3] {
    [
        rgb[0] as f32 / 255.0,
        rgb[1] as f32 / 255.0,
        rgb[2] as f32 / 255.0,
    ]
}

#[inline] pub fn srgb_to_rgb(srgb: [f32; 3]) -> [u8; 3] {
    [
        (srgb[0] * 255.0) as u8,
        (srgb[1] * 255.0) as u8,
        (srgb[2] * 255.0) as u8,
    ]
}

// PUBLIC API
pub fn contrast<T>(rgb: T, amount: f32) -> T 
    where 
        T: Into<[u8; 3]> + From<[u8; 3]> 
{
    T::from(_contrast_u8(rgb.into(), amount))
}

pub fn gradient_map<T, U>(rgb: T, gradient: &[(U, f32)]) -> Option<U>
    where 
        T: Into<[u8; 3]> + From<[u8; 3]>,
        U: Copy + Clone + Into<Srgb> + From<Srgb>
{
    _gradient_map_u8(rgb.into(), gradient)
}

pub fn quantize_hue<T>(rgb: T, hues: &[f32]) -> T 
    where 
        T: Into<[u8; 3]> + From<[u8; 3]> 
{
    T::from(_quantize_hue_u8(rgb.into(), hues))
}

pub fn brighten<T>(rgb: T, factor: f32) -> T 
    where 
        T: Into<[u8; 3]> + From<[u8; 3]> 
{
    T::from(_brighten_u8(rgb.into(), factor))
}

pub fn saturate<T>(rgb: T, factor: f32) -> T 
    where 
        T: Into<[u8; 3]> + From<[u8; 3]> 
{
    T::from(_saturate_u8(rgb.into(), factor))
}

pub fn shift_hue<T>(rgb: T, degrees: f32) -> T 
    where 
        T: Into<[u8; 3]> + From<[u8; 3]> 
{
    T::from(_shift_hue_u8(rgb.into(), degrees))
}

// PRIVATE API
fn _contrast_u8(rgb: [u8; 3], amount: f32) -> [u8; 3] {
    let mut color = Srgb::from(rgb).into_format::<f32>();
    color.red = (((color.red - 0.5) * amount) + 0.5).clamp(0.0, 1.0);
    color.blue = (((color.blue - 0.5) * amount) + 0.5).clamp(0.0, 1.0);
    color.green = (((color.green - 0.5) * amount) + 0.5).clamp(0.0, 1.0);
    Srgb::from_color(color).into_format().into()
}

fn _gradient_map_u8<U>(rgb: [u8; 3], gradient: &[(U, f32)]) -> Option<U> 
    where U: Copy + Clone + Into<Srgb> + From<Srgb>
{
    let mut color = Srgb::from(rgb).into_format::<f32>();
    let color = Lch::from_color(color);
    let l = color.l / 100.0;

    let mut gradient = Vec::from(gradient.clone());
    gradient.sort_by(|a, b|
        a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let index = gradient.iter().position(|(_, threshold)| l < *threshold);

    if index.is_none() { return None };

    let index = index.unwrap();

    let prev_col = gradient.get(index - 1);
    let curr_col = gradient.get(index);

    if prev_col.and(curr_col).is_some() {
        let (c_col, c_th) = curr_col.unwrap();
        let (p_col, p_th) = prev_col.unwrap();

        let c_col: Srgb = (*c_col).into();
        let p_col: Srgb = (*p_col).into();

        let c_dist = c_th - l;
        let p_dist = l - p_th;

        let c_ratio = 1.0 - (c_dist / (c_dist + p_dist));
        let p_ratio = 1.0 - (p_dist / (c_dist + p_dist));

        let (c_r, c_g, c_b) = c_col.into_components();
        let (p_r, p_g, p_b) = p_col.into_components();

        let (new_r, new_g, new_b) = (
            (c_ratio * c_r + p_ratio * p_r),
            (c_ratio * c_g + p_ratio * p_g),
            (c_ratio * c_b + p_ratio * p_b),
        );

        Some(U::from(Srgb::from_components((new_r, new_g, new_b,))))

    } else if curr_col.is_some() {
        curr_col.map(|tup| tup.0)
    } else {
        None
    }
}

pub fn _quantize_hue_u8(rgb: [u8; 3], hues: &[f32]) -> [u8; 3] {
    let color = Srgb::from(rgb).into_format::<f32>();
    let mut color = Lch::from_color(color);
    color.set_hue(utils::quantize_hue(color.hue.into_degrees(), hues));
    Srgb::from_color(color).into_format().into()
}

pub fn _brighten_u8(rgb: [u8; 3], factor: f32) -> [u8; 3] {
    let color = Srgb::from(rgb).into_format::<f32>();
    let mut color = Lch::from_color(color);

    if factor >= 0.0 {
        color = color.lighten(factor);
    } else {
        color = color.darken(factor.abs());
    };

    Srgb::from_color(color).into_format().into()
}

pub fn _saturate_u8(rgb: [u8; 3], factor: f32) -> [u8; 3] {
    let color = Srgb::from(rgb).into_format::<f32>();
    let mut color = Lch::from_color(color);

    color.chroma = if factor >= 0.0 {
        color.chroma + (CHROMA_BOUND - color.chroma) * factor
    } else {
        color.chroma + (color.chroma) * factor
    };
    Srgb::from_color(color).into_format().into()
}

pub fn _shift_hue_u8(rgb: [u8; 3], hue: f32) -> [u8; 3] {
    let color = Srgb::from(rgb).into_format::<f32>();
    let mut color = Lch::from_color(color);
    color = color.shift_hue(hue);
    Srgb::from_color(color).into_format().into()
}