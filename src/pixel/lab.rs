// massive thanks to https://github.com/antimatter15/rgb-lab for providing a good example of an implementation
use super::rgb::RgbPixel;

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
        let (r, g, b) = rgb.get();
        let (mut r, mut g, mut b) = (
            r as f32 / 255.0,
            b as f32 / 255.0,
            g as f32 / 255.0,
        );

        let update_channel = |num: f32| 
            if num > 0.04045 {
                ((num + 0.055) / 1.055).powf(2.4)
            } else {
                num / 12.92
            };

        r = update_channel(r);
        g = update_channel(g);
        b = update_channel(b);

        let mut x = (r * 0.4124 + g * 0.3576 + b * 0.1805) / 0.95047;
        let mut y = (r * 0.2126 + g * 0.7152 + b * 0.0722) / 1.00000;
        let mut z = (r * 0.0193 + g * 0.1192 + b * 0.9505) / 1.08883;

        let update_component = |num: f32|
            if num > 0.008856 {
                num.powf(1.0/3.0)
            } else {
                num * 7.787 + 16.0/116.0
            };

            x = update_component(x);
            y = update_component(y);
            z = update_component(z);

        LabPixel(
            (116.0 * y) - 16.0,
            500.0 * (x - y),
            200.0 * (y - z),
        )
    }

    pub fn as_rgb(&self) -> RgbPixel {
        let (l, a, b) = self.get();

        let mut y = (l + 16.0) / 116.0;
        let mut x = a / 500.0 + y;
        let mut z = y - b / 200.0;

        let update_component = |num: f32|
            if num.powf(3.0) > 0.008856 {
                num.powf(3.0)
            } else {
                (num - 16.0/116.0) / 7.787
            };

        x = update_component(x);
        y = update_component(y);
        z = update_component(z);

        let mut r = x *  3.2406 + y * -1.5372 + z * -0.4986;
        let mut g = x * -0.9689 + y *  1.8758 + z *  0.0415;
        let mut b = x *  0.0557 + y * -0.2040 + z *  1.0570;

        let update_channel = |num: f32|
            if num > 0.0031308 {
                (1.055 * num.powf(1.0/2.4)) - 0.055
            } else {
                12.92 * num
            };

        r = update_channel(r);
        g = update_channel(g);
        b = update_channel(b);

        RgbPixel::new(
            (r.clamp(0.0, 1.0) * 255.0) as u8,
            (g.clamp(0.0, 1.0) * 255.0) as u8,
            (b.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::pixel::{rgb::RgbPixel, lab::LabPixel};


    #[test]
    fn from_rgb() {
        let lab = LabPixel::from(RgbPixel::new(255, 0, 0));
        assert_eq!(lab.0, 53.23288);
        assert_eq!(lab.1, 80.10930);
        assert_eq!(lab.2, 67.22008);
    }
}