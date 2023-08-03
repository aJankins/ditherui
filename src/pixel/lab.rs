use super::{rgb::RgbPixel, conversions::{chain_conversions, rgb_to_xyz_d65, xyz_d65_to_xyz_d50, xyz_d50_to_lab, lab_to_xyz_d50, xyz_d50_to_xyz_d65, xyz_d65_to_rgb}};

/*
    WARNING!
    This may not be 100% accurate. Converting an image from RGB to LAB and back results in some errors.
*/

#[derive(Debug, Clone, Copy)]
pub struct LabPixel(f32, f32, f32);

impl From<(f32, f32, f32)> for LabPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        let (l, a, b) = value;
        LabPixel(l, a, b)
    }
}

impl From<RgbPixel> for LabPixel {
    fn from(value: RgbPixel) -> Self {
        Self::from_rgb(&value)
    }
}

impl Into<RgbPixel> for LabPixel {
    fn into(self) -> RgbPixel {
        self.as_rgb()
    }
}

impl LabPixel {
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn distance_from(&self, other: &LabPixel) -> f32 {
        let delta_l = self.0 - other.0;
        let delta_a = self.1 - other.1;
        let delta_b = self.2 - other.2;

        let c1 = self.1.powf(2.0) + self.2.powf(2.0);
        let c2 = other.1.powf(2.0) + other.2.powf(2.0);

        let delta_c = c1 - c2;
        let delta_h = delta_a.powf(2.0) + delta_b.powf(2.0) + delta_c.powf(2.0);
        let delta_h = if delta_h < 0.0 { 0.0 } else {delta_h.sqrt()};

        let sc = 1.0 + 0.045 * c1;
        let sh = 1.0 + 0.015 * c1;

        let delta_l_kl_sl = delta_l / (1.0);
        let delta_c_kc_sc = delta_c / sc;
        let delta_h_kh_sh = delta_h / sh;

        let i = delta_l_kl_sl.powf(2.0) + delta_c_kc_sc.powf(2.0) + delta_h_kh_sh.powf(2.0);
        
        if i < 0.0 {
            0.0
        } else {
            i.sqrt()
        }
    }

    pub fn from_rgb(rgb: &RgbPixel) -> LabPixel {
        chain_conversions(rgb.get(), &[
            rgb_to_xyz_d65,
            xyz_d65_to_xyz_d50,
            xyz_d50_to_lab,
        ]).into()
    }

    pub fn as_rgb(&self) -> RgbPixel {
        chain_conversions(self.get(), &[
            lab_to_xyz_d50,
            xyz_d50_to_xyz_d65,
            xyz_d65_to_rgb,
        ]).into()
    }
}