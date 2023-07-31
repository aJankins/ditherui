use image::Rgb;

#[derive(Debug)]
pub struct MonoPixel(u8);

pub const TWO_BIT: &'static [MonoPixel] = &[MonoPixel(0), MonoPixel(255)];

impl From<u8> for MonoPixel {
    fn from(value: u8) -> Self {
        MonoPixel(value)
    }
}

impl From<&Rgb<u8>> for MonoPixel {
    fn from(value: &Rgb<u8>) -> Self {
        let [r, g, b] = value.0;
        let luminance = (r.max(g).max(b) as u16 + r.min(g).min(b) as u16) / 2;
        MonoPixel(
            luminance as u8
        )
    }
}



impl MonoPixel {
    pub fn add_error(self, error: i32) -> MonoPixel {
        MonoPixel((self.0 as i32 + error).min(255).max(0) as u8)
    }

    pub fn quantize(&self, palette: &[MonoPixel]) -> MonoPixel {
        let mut closest_dist = u16::MAX;
        let mut closest_col = self;

        for colour in palette.iter() {
            let distance = (colour.0 as i16 - self.0 as i16).abs() as u16;
            if distance < closest_dist {
                closest_col = colour;
                closest_dist = distance;
            }
        }

        // println!("{} ---- quantized to ---> {}", self.0, closest_col.get());

        closest_col.get().into()
    }

    pub fn get_error(&self, other: &MonoPixel) -> i32 {
        self.0 as i32 - other.0 as i32
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}