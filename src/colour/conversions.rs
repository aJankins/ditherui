// constants

/// Converts LCH to LAB.
/// 
/// The expected ranges for LCH are `(0.0~100.0, 0.0~150.0, 0.0~360.0)`
/// 
/// The returned LAB values have the following ranges: `(0.0~100.0, -125.0~125.0, -125.0~125.0)`
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