use super::{lab::LabPixel, rgb::RgbPixel, conversions::{lab_to_lch, lch_to_lab}};

/*
    WARNING!
    This may not be 100% accurate. Converting an image from RGB to LCH and back results in some errors.
*/

#[derive(Debug, Clone, Copy)]
pub struct LchPixel(pub f32, pub f32, pub f32);

impl From<(f32, f32, f32)> for LchPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        let (l, c, h) = value;
        LchPixel(l, c, h)
    }
}

impl From<RgbPixel> for LchPixel {
    fn from(value: RgbPixel) -> Self {
        Self::from_lab(&LabPixel::from_rgb(&value))
    }
}

impl From<LabPixel> for LchPixel {
    fn from(value: LabPixel) -> Self {
        Self::from_lab(&value)
    }
}

impl Into<RgbPixel> for LchPixel {
    fn into(self) -> RgbPixel {
        self.as_lab().as_rgb()
    }
}

impl Into<LabPixel> for LchPixel {
    fn into(self) -> LabPixel {
        self.as_lab()
    }
}

impl LchPixel {
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn from_lab(lab: &LabPixel) -> LchPixel {
        lab_to_lch(lab.get()).into()
    }

    pub fn as_lab(&self) -> LabPixel {
        lch_to_lab(self.get()).into()
    }

    pub fn add_luma(&mut self, luma: f32) -> &mut Self {
        self.0 = (self.0 + luma).clamp(0.0, 100.0);
        self
    }

    pub fn add_chroma(&mut self, chroma: f32) -> &mut Self {
        self.1 = (self.1 + chroma).clamp(0.0, 132.0);
        self
    }

    pub fn add_hue(&mut self, hue: f32) -> &mut Self {
        self.2 = self.2 + hue;
        self
    }
}