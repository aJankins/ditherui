// constants
const D50_WHITE: [f32; 3] = [
    0.3457 / 0.3585,
    1.00000,
    (1.0 - 0.3457 - 0.3585) / 0.3585,
];

const D65_WHITE: [f32; 3] = [
    0.3127 / 0.3290,
    1.00000,
    (1.0 - 0.3126 - 0.3290) / 0.3290,
];

// RGB -> HSL -> RGB

pub fn rgb_to_hsl(rgb: (f32, f32, f32)) -> (f32, f32, f32) {
    let (r, g, b) = rgb;

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

    (hue, saturation, lightness)
}

pub fn hsl_to_rgb(hsl: (f32, f32, f32)) -> (f32, f32, f32) {
    let (mut h, s, l) = hsl;
    let chroma = (1.0 - (2.0 * l - 1.0).abs()) * s;

    let hue_degree = (loop {
        if h >= 0.0 {
            break h % 360.0;
        }
        h = h + 360.0
    } % 360.0) / 60.0;

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
            "Hue degree should be between 0 and 6 - was actually: {}",
            hue_degree
        )
    };

    let m = l - (chroma / 2.0);

    (
        (r1 + m),
        (g1 + m),
        (b1 + m),
    )
}

// RGB -> XYZ_D65 -> RGB

pub fn rgb_to_xyz_d65(rgb: (f32, f32, f32)) -> (f32, f32, f32) {
    let (r, g, b) = rgb;

    let x = 
        r * 0.41239079926595934 +
        g * 0.35758433938387800 +
        b * 0.18048078840183430;

    let y = 
        r * 0.21263900587151027 +
        g * 0.71516867876775600 +
        b * 0.07219231536073371;

    let z = 
        r * 0.01933081871559182 +
        g * 0.11919477979462598 +
        b * 0.95053215224966070;

    (x, y, z)
}

pub fn xyz_d65_to_rgb(xyz: (f32, f32, f32)) -> (f32, f32, f32) {
    let (x, y, z) = xyz;

    let r = 
        x as f32 *  3.24096994190452260 +
        y as f32 * -1.53738317757009400 +
        z as f32 * -0.49861076029300340;

    let g = 
        x as f32 * -0.96924363628087960 +
        y as f32 *  1.87596750150772020 +
        z as f32 *  0.04155505740717559;

    let b = 
        x as f32 *  0.05563007969699366 +
        y as f32 * -0.20397695888897652 +
        z as f32 *  1.05697151424287860;

    (r, g, b)
}

// XYZ_D65 -> XYZ_D50 -> XYZ_D65
pub fn xyz_d65_to_xyz_d50(xyz_d65: (f32, f32, f32)) -> (f32, f32, f32) {
    let (x, y, z) = xyz_d65;

    (
        x * 1.0479298208405488 + y * 0.022946793341019088 + z * -0.05019222954313557,
        x * 0.029627815688159344 + y *  0.990434484573249 + z * -0.01707382502938514,
        x * -0.009243058152591178 + y * 0.015055144896577895 + z * 0.7518742899580008,
    )
}

pub fn xyz_d50_to_xyz_d65(xyz_d50: (f32, f32, f32)) -> (f32, f32, f32) {
    let (x, y, z) = xyz_d50;

    (
        x * 0.9554734527042182 + y * -0.023098536874261423 + z * 0.0632593086610217,
        x * -0.028369706963208136 + y * 1.0099954580058226 + z * 0.021041398966943008,
        x * 0.012314001688319899 + y * -0.020507696433477912 + z * 1.3303659366080753,
    )
}

// XYZ_D50 -> LAB -> XYZ_D50
pub fn xyz_d50_to_lab(xyz_d50: (f32, f32, f32)) -> (f32, f32, f32) {
    const EPSILON: f32 = 216.0/24389.0;
    const K: f32 = 24389.0/27.0;

    let (x, y, z) = xyz_d50;
    
    let scale_to_white = |num: f32, i: usize| num / D50_WHITE[i];
    let (x, y, z) = (
        scale_to_white(x, 0),
        scale_to_white(y, 1),
        scale_to_white(z, 2),
    );

    let compute_f = |num: f32| if num > EPSILON {
        num.cbrt()
    } else {
        (K * num + 16.0) / 116.0
    };

    let f = (compute_f(x), compute_f(y), compute_f(z));

    (
        (116.0 * f.1) - 16.0,
        500.0 * (f.0 - f.1),
        200.0 * (f.1 - f.2),
    )
}

pub fn lab_to_xyz_d50(lab: (f32, f32, f32)) -> (f32, f32, f32) {
    const EPSILON3: f32 = 24.0/116.0;
    const K: f32 = 24389.0/27.0;

    let mut f = [0.0_f32; 3];
    f[1] = (lab.0 + 16.0) / 116.0;
    f[0] = (lab.1 / 500.0) + f[1];
    f[2] = f[1] - (lab.2 / 200.0);

    let (x, y, z) = (
        if f[0]    > EPSILON3 { f[0].powi(3)                   } else { (116.0 * f[0] - 16.0) / K },
        if lab.0   > 8.0      { ((lab.0+16.0) / 116.0).powi(3) } else { lab.0 / K                 },
        if f[2]    > EPSILON3 { f[2].powi(3)                   } else { (116.0 * f[2] - 16.0) / K},
    );

    let scale_to_white = |num: f32, i: usize| num * D50_WHITE[i];

    (
        scale_to_white(x, 0),
        scale_to_white(y, 1),
        scale_to_white(z, 2),
    )
}

// LAB -> LCH -> LAB

pub fn lab_to_lch(lab: (f32, f32, f32)) -> (f32, f32, f32) {
    let (l, a, b) = lab;
    const EPSILON: f32 = 0.02;

    let hue = if a.abs() < EPSILON && b.abs() < EPSILON {
        f32::NAN
    } else {
        b.atan2(a) * 180.0 / std::f32::consts::PI
    };

    (
        l,
        (a.powi(2) + b.powi(2)).sqrt(),
        ((hue % 360.0) + 360.0) % 360.0
    )
}

pub fn lch_to_lab(lch: (f32, f32, f32)) -> (f32, f32, f32) {
    let (l,mut c, mut h) = lch;
    c = c.max(0.0);

    if h.is_nan() {
        h = 0.0;
    }

    (
        l,
        c * (h * std::f32::consts::PI / 180.0).cos(),
        c * (h * std::f32::consts::PI / 180.0).sin(),
    )
}

// utils

pub fn chain_conversions(input: (f32, f32, f32), conversions: &[fn((f32, f32, f32)) -> (f32, f32, f32)]) -> (f32, f32, f32) {
    let mut result = input;
    for func in conversions.iter() {
        result = func(result);
    }
    result
}