#[inline] pub fn collapse_angle(angle: f32) -> f32 {
    ((angle % 360.0) + 360.0) % 360.0
}

pub fn quantize_hue(original_hue: f32, hues: &[f32]) -> f32 {
    let mut closest_dist = f32::MAX;
    let pixel_hue = collapse_angle(original_hue);
    let mut current_hue = pixel_hue;

    for hue in hues.iter() {
        let normalized = collapse_angle(*hue);
        let distance = (normalized - pixel_hue).abs();
        if distance < closest_dist {
            closest_dist = distance;
            current_hue = normalized;
        }
    }

    current_hue
}