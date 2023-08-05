use palette::{IntoColor, Lch, Hsl, Srgb, FromColor};

use super::utils::GradientMethod;

pub trait IntoGradient: Sized + IntoColor<Lch> + IntoColor<Hsl> + Copy {
    fn build_gradient(self, shades: u16, method: GradientMethod) -> Vec<Self>;

    fn build_gradient_hsl(self, shades: u16) -> Vec<Self> {
        self.build_gradient(shades, GradientMethod::HSL)
    }

    fn build_gradient_lch(self, shades: u16) -> Vec<Self> {
        self.build_gradient(shades, GradientMethod::LCH)
    }
}

impl<C: Sized 
    + IntoColor<Lch> + IntoColor<Hsl> 
    + FromColor<Lch> + FromColor<Hsl>
    + Copy + FromColor<Srgb>> IntoGradient for C 
{
    fn build_gradient(self, shades: u16, method: GradientMethod) -> Vec<Self> {
        let fractional_lch = 100.0 / shades as f32 + 1.0;
        let fractional_hsl = 1.0 / shades as f32 + 1.0;
        let luma_modify: Box<dyn FnMut(u16) -> Self> = match method {
            GradientMethod::LCH => {
                Box::new(|i : u16| {
                    let mut color: Lch = self.into_color();
                    color.l = i as f32 * fractional_lch;
                    Self::from_color(color)
                })
            },
            GradientMethod::HSL => {
                Box::new(|i : u16| {
                    let mut color: Hsl = self.into_color();
                    color.lightness = i as f32 * fractional_hsl;
                    Self::from_color(color)
                })
            },
        };
    
        (1..shades)
            .into_iter()
            .map(luma_modify)
            .collect()
    }
} 