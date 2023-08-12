use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Frame, Rgba};
use palette::Srgb;

use crate::{Effect, utils::numops::map_to_2d, EffectInput};

use super::{
    basic::basic_dither,
    bayer::bayer_dither,
    error::{
        floyd_steinberg, jarvis_judice_ninke, atkinson, burkes, stucki, sierra, RgbImageRepr, RgbaImageRepr
    },
};

/// Dithering algorithms. Each one accepts a *palette* - aka a list of `RgbPixel`s that colours should
/// be quantized to.
pub enum Dither<'a> {
    /// The basic error propagation method.
    ///
    /// Results in the worst quality output, but included for curiosity's sake
    Basic(&'a [Srgb]),
    /// Floyd Steinberg dithering.
    FloydSteinberg(&'a [Srgb]),
    /// Jarvis Judice Ninke dithering.
    JarvisJudiceNinke(&'a [Srgb]),
    /// Stucki dithering.
    Stucki(&'a [Srgb]),
    /// Atkinson dithering.
    Atkinson(&'a [Srgb]),
    /// Burkes dithering.
    Burkes(&'a [Srgb]),
    /// Sierra dithering.
    Sierra(&'a [Srgb]),
    /// Sierra two-row dithering.
    SierraTwoRow(&'a [Srgb]),
    /// Sierra lite dithering.
    SierraLite(&'a [Srgb]),
    /// Bayer / Ordered dithering.
    ///
    /// Accepts the matrix size. 1 results in no dithering, and 4+ is recommended.
    /// Isn't as accurate as the error propagation methods, but can be stylistically preferred.
    Bayer(usize, &'a [Srgb]),
}

impl<'a> EffectInput<Dither<'a>> for RgbImageRepr {
    fn run_through(&self, algorithm: &Dither) -> Self {
        let mut output = self.clone();
        match algorithm {
            Dither::Basic(palette)                => basic_dither(&mut output, palette),
            Dither::FloydSteinberg(palette)       => floyd_steinberg::dither_rgb(&mut output, palette),
            Dither::JarvisJudiceNinke(palette)    => jarvis_judice_ninke::dither_rgb(&mut output, palette),
            Dither::Stucki(palette)               => stucki::dither_rgb(&mut output, palette),
            Dither::Atkinson(palette)             => atkinson::dither_rgb(&mut output, palette),
            Dither::Burkes(palette)               => burkes::dither_rgb(&mut output, palette),
            Dither::Sierra(palette)               => sierra::dither_rgb(&mut output, palette),
            Dither::SierraTwoRow(palette)         => sierra::two_row::dither_rgb(&mut output, palette),
            Dither::SierraLite(palette)           => sierra::lite::dither_rgb(&mut output, palette),
            Dither::Bayer(n, palette)     => bayer_dither(&mut output, *n, palette),
        }
        output
    }
}

impl<'a> EffectInput<Dither<'a>> for RgbaImageRepr {
    fn run_through(&self, algorithm: &Dither) -> Self {
        let ys = self.len();
        let xs = self.get(0).map_or(0, |row| row.len());

        if xs == 0 || ys == 0 {
            return self.clone();
        }

        // split into Rgb and Alpha representations.
        let mut rgb_img = vec![vec![[0_u8; 3]; xs]; ys];
        let mut alpha_values = vec![vec![0_u8; xs]; ys];
        // prepare output
        let mut actual_img = vec![vec![[0_u8; 4]; xs]; ys];

        for y in 0..ys {
            for x in 0..xs {
                if self.get(y).and_then(|row| row.get(x)).is_none() { continue; }
                let [r,g,b,a] = self[y][x]; 
                rgb_img[y][x] = [r, g, b];
                alpha_values[y][x] = a;
            }
        }
    

        rgb_img = rgb_img.run_through(algorithm);

        // recombine rgb with alpha
        for y in 0..ys {
            for x in 0..xs {
                actual_img[y][x] = [
                    rgb_img[y][x][0],
                    rgb_img[y][x][1],
                    rgb_img[y][x][2],
                    alpha_values[y][x],
                ];
            }
        }
        
        actual_img
    }
}

impl<'a> EffectInput<Dither<'a>> for ImageBuffer<Rgb<u8>, Vec<u8>> {
    fn run_through(&self, effect: &Dither<'a>) -> Self {
        let (xs, ys) = self.dimensions();
        let (xs, ys) = (xs as usize, ys as usize);
    
        let mut img_matrix = vec![vec![[0_u8; 3]; xs]; ys];
    
        for (i, pixel) in self.pixels().into_iter().enumerate() {
           let (x, y) = map_to_2d(i, xs);
           img_matrix[y][x] = pixel.0;
        }
    
        img_matrix = img_matrix.run_through(effect);

        let ydim = img_matrix.len() as u32;
        let xdim = img_matrix.get(0).map(|row| row.len()).unwrap_or(0) as u32;
    
        ImageBuffer::from_fn(xdim, ydim, |x, y| {
            image::Rgb(img_matrix[y as usize][x as usize])
        })
    }
}

impl<'a> EffectInput<Dither<'a>> for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn run_through(&self, effect: &Dither<'a>) -> Self {
        let (xs, ys) = self.dimensions();
        let (xs, ys) = (xs as usize, ys as usize);
    
        let mut img_matrix = vec![vec![[0_u8; 4]; xs]; ys];
    
        for (i, pixel) in self.pixels().into_iter().enumerate() {
           let (x, y) = map_to_2d(i, xs);
           img_matrix[y][x] = pixel.0;
        }
    
        img_matrix = img_matrix.run_through(effect);

        let ydim = img_matrix.len() as u32;
        let xdim = img_matrix.get(0).map(|row| row.len()).unwrap_or(0) as u32;
    
        ImageBuffer::from_fn(xdim, ydim, |x, y| {
            image::Rgba(img_matrix[y as usize][x as usize])
        })
    }
}

impl<'a> EffectInput<Dither<'a>> for DynamicImage {
    fn run_through(&self, algorithm: &Dither) -> Self {
        match self {
            DynamicImage::ImageRgb8(img) 
                => DynamicImage::from(img.run_through(algorithm)),

            DynamicImage::ImageRgba8(img) 
                => DynamicImage::from(img.run_through(algorithm)),

            _ => DynamicImage::ImageRgb8(self.clone().into_rgb8().run_through(algorithm))
        }
    }
}

impl<'a> EffectInput<Dither<'a>> for Frame {
    fn run_through(&self, effect: &Dither<'a>) -> Self {
        let left = self.left();
        let top = self.top();
        let delay = self.delay();

        let new_buf = self.buffer().run_through(effect);
        Frame::from_parts(new_buf, left, top, delay)
    }
}