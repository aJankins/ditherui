use super::conversions::lch_to_lab;

pub fn ciede2000(col_a: (f32, f32, f32), col_b: (f32, f32, f32)) -> f32 {
    // set up constants for formula
    // these are usually unity (1)
    let k_l: f32 = 1.0;
    let k_c: f32 = 1.0;
    let k_h: f32 = 1.0;

    // get LAB values - needed for formula
    let (l_1, a_1, b_1) = lch_to_lab(col_a);
    let (l_2, a_2, b_2) = lch_to_lab(col_b);

    // set up variables for formula
    let delta_l_mark = col_b.0 - col_a.0;

    let avg_l = (col_b.0 + col_a.0) / 2.0;
    let avg_c = (col_b.1 + col_a.1) / 2.0;

    let c_7_mul = 1.0 - (avg_c.powi(7) / (avg_c.powi(7) + 25_f32.powi(7)).sqrt());
    let a_1_mark = a_1 + (a_1 / 2.0) * c_7_mul;
    let a_2_mark = a_2 + (a_2 / 2.0) * c_7_mul;

    let c_1_mark = (a_1_mark.powi(2) + b_1.powi(2)).sqrt();
    let c_2_mark = (a_2_mark.powi(2) + b_2.powi(2)).sqrt();

    let delta_c_mark = c_2_mark - c_1_mark;
    let avg_c_mark = (c_2_mark + c_1_mark) / 2.0;

    let h_1_mark = b_1.atan2(a_1_mark).to_degrees() % 360.0;
    let h_2_mark = b_1.atan2(a_1_mark).to_degrees() % 360.0;

    let abs_diff_h_marks = (h_1_mark - h_2_mark).abs();
    let delta_h_mark = 
        if c_1_mark == 0.0 || c_2_mark == 0.0 {
            0.0
        } else if abs_diff_h_marks <= 180.0 {
            h_2_mark - h_1_mark
        } else if abs_diff_h_marks > 180.0 && h_2_mark <= h_1_mark {
            h_2_mark - h_1_mark + 360.0
        } else {
            h_2_mark - h_1_mark - 360.0
        };

    let delta_big_h_mark = 2.0 * (c_1_mark * c_2_mark).sqrt() * (delta_h_mark / 2.0).to_radians().sin();
    let avg_big_h_mark =
        if c_1_mark == 0.0 || c_2_mark == 0.0 {
            h_1_mark + h_2_mark
        } else if abs_diff_h_marks <= 180.0 {
            (h_1_mark + h_2_mark) / 2.0
        } else if abs_diff_h_marks > 180.0 && h_1_mark + h_2_mark < 360.0 {
            (h_1_mark + h_2_mark + 360.0) / 2.0
        } else {
            (h_1_mark + h_2_mark - 360.0) / 2.0
        };

    let t = 1.0
        - 0.17 * (avg_big_h_mark - 30.0).to_radians().cos()
        + 0.24 * (avg_big_h_mark * 2.0).to_radians().cos()
        + 0.32 * (avg_big_h_mark * 3.0 + 6.0).to_radians().cos()
        - 0.20 * (avg_big_h_mark * 4.0 - 63.0).to_radians().cos();

    let s_l = 1.0
        + (0.015 * (avg_l - 50.0).powi(2))
        / (20.0 + (avg_l - 50.0).powi(2));

    let s_c = 1.0 + 0.045 * avg_c_mark;
    let s_h = 1.0 + 0.015 * avg_c_mark * t;

    let r_t = -2.0
        * (avg_c_mark.powi(7) / (avg_c_mark.powi(7) + 25_f32.powi(7))).sqrt()
        * (60.0 * (-1.0 * ((avg_big_h_mark - 275.0) / 25.0).powi(2)).exp()).to_radians().sin();

    // the actual formula
    (
          (delta_l_mark / (k_l * s_l)).powi(2)
        + (delta_c_mark / (k_c * s_c)).powi(2)
        + (delta_big_h_mark / (k_h * s_h)).powi(2)
        + r_t
            * (delta_c_mark / (k_c * s_c))
            * (delta_big_h_mark / (k_h * s_h))
    ).sqrt()
}