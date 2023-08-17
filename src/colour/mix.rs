use palette::{IntoColor, rgb::Rgb, FromColor, Hsl, Lch, Lab, Oklch};


pub enum MixMethod {
    RGB,
    HSL,
    LAB,
    LCH,
    OKLCH,
}

pub trait MixColourRgb: Sized + IntoColor<Rgb> + FromColor<Rgb> + Copy {
    fn mix_rgb<T>(self, other: T) -> Self where
        T: Sized + IntoColor<Rgb> + FromColor<Rgb> + Copy
    {
        let rgb1: Rgb = self.into_color();
        let rgb2: Rgb = other.into_color();

        let (r1, g1, b1) = rgb1.into_components();
        let (r2, g2, b2) = rgb2.into_components();

        Rgb::new(
            (r1 + r2) / 2.0,
            (g1 + g2) / 2.0,
            (b1 + b2) / 2.0,
        ).into_color()
    }
}

pub trait MixColourHsl: Sized + IntoColor<Hsl> + FromColor<Hsl> + Copy {
    fn mix_hsl<T>(self, other: T) -> Self where
        T: Sized + IntoColor<Hsl> + FromColor<Hsl> + Copy
    {
        let hsl1: Hsl = self.into_color();
        let hsl2: Hsl = other.into_color();

        let (h1, s1, l1) = hsl1.into_components();
        let (h2, s2, l2) = hsl2.into_components();

        Hsl::new(
            (h1.into_degrees() + h2.into_degrees()) / 2.0,
            (s1 + s2) / 2.0,
            (l1 + l2) / 2.0,
        ).into_color()
    }
}

pub trait MixColourLab: Sized + IntoColor<Lab> + FromColor<Lab> + Copy {
    fn mix_lab<T>(self, other: T) -> Self where
        T: Sized + IntoColor<Lab> + FromColor<Lab> + Copy
    {
        let lab1: Lab = self.into_color();
        let lab2: Lab = other.into_color();

        let (l1, a1, b1) = lab1.into_components();
        let (l2, a2, b2) = lab2.into_components();

        Lab::new(
            (l1 + l2) / 2.0,
            (a1 + a2) / 2.0,
            (b1 + b2) / 2.0,
        ).into_color()
    }
}

pub trait MixColourLch: Sized + IntoColor<Lch> + FromColor<Lch> + Copy {
    fn mix_lch<T>(self, other: T) -> Self where
        T: Sized + IntoColor<Lch> + FromColor<Lch> + Copy
    {
        let lch1: Lch = self.into_color();
        let lch2: Lch = other.into_color();

        let (l1, c1, h1) = lch1.into_components();
        let (l2, c2, h2) = lch2.into_components();

        Lch::new(
            (l1 + l2) / 2.0,
            (c1 + c2) / 2.0,
            (h1.into_degrees() + h2.into_degrees()) / 2.0,
        ).into_color()
    }
}

pub trait MixColourOklch: Sized + IntoColor<Oklch> + FromColor<Oklch> + Copy {
    fn mix_oklch<T>(self, other: T) -> Self where
        T: Sized + IntoColor<Oklch> + FromColor<Oklch> + Copy
    {
        let oklch1: Oklch = self.into_color();
        let oklch2: Oklch = other.into_color();

        let (l1, c1, h1) = oklch1.into_components();
        let (l2, c2, h2) = oklch2.into_components();

        Oklch::new(
            (l1 + l2) / 2.0,
            (c1 + c2) / 2.0,
            (h1.into_degrees() + h2.into_degrees()) / 2.0,
        ).into_color()
    }
}

pub trait MixColour: MixColourRgb + MixColourHsl + MixColourLab + MixColourLch + MixColourOklch {
    fn mix<T>(self, other: T, method: MixMethod) -> Self where
        T: MixColourRgb + MixColourHsl + MixColourLab + MixColourLch + MixColourOklch
    {
        match method {
            MixMethod::RGB => self.mix_rgb(other),
            MixMethod::HSL => self.mix_hsl(other),
            MixMethod::LAB => self.mix_lab(other),
            MixMethod::LCH => self.mix_lch(other),
            MixMethod::OKLCH => self.mix_oklch(other),
        }
    }
}

// general implementations
impl<C> MixColourRgb for C where
    C: Sized + IntoColor<Rgb> + FromColor<Rgb> + Copy {}

impl<C> MixColourHsl for C where
    C: Sized + IntoColor<Hsl> + FromColor<Hsl> + Copy {}

impl<C> MixColourLab for C where
    C: Sized + IntoColor<Lab> + FromColor<Lab> + Copy {}

impl<C> MixColourLch for C where
    C: Sized + IntoColor<Lch> + FromColor<Lch> + Copy {}

impl<C> MixColourOklch for C where
    C: Sized + IntoColor<Oklch> + FromColor<Oklch> + Copy {}

impl<C> MixColour for C where
    C: MixColourRgb + MixColourHsl + MixColourLab + MixColourLch + MixColourOklch {}