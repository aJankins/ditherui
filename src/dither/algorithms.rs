use image::{DynamicImage, Pixel};

use crate::utils::u8ops::average;

pub enum Algorithms {
    BasicMono,
    FloydSteinbergMono,
    JarvisJudiceNinkeMono,
    StuckiMono,
    AtkinsonMono,
}

impl Algorithms {
    pub fn dither(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Self::BasicMono => basic_mono_dither(image),
            Self::FloydSteinbergMono => floyd_steinberg_mono_dither(image),
            Self::JarvisJudiceNinkeMono => jarvis_judice_ninke_mono_dither(image),
            Self::StuckiMono => stucki_mono_dither(image),
            Self::AtkinsonMono => atkinson_mono_dither(image),
        }
    }
}

fn basic_mono_dither(image: DynamicImage) -> DynamicImage {
    let collapser = |num: i16| if num < 128 { 0 } else { 255 };

    let mut error = 0;
    let mut rgb8_image = image.into_rgb8();

    for pixel in rgb8_image.pixels_mut() {
        let mono = average(pixel.channels()) as i16 + error;
        let threshold = collapser(mono);

        error = mono - threshold;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

fn floyd_steinberg_mono_dither(image: DynamicImage) -> DynamicImage {
    let collapser = |num: i64| if num < 128 { 0 } else { 255 };

    let mut rgb8_image = image.into_rgb8();
    let (xdim, ydim) = rgb8_image.dimensions();
    let mut error_matrix = vec![vec![0 as i64; (xdim+1) as usize]; (ydim+1) as usize];


    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let mono = average(pixel.channels()) as i64 + *(&error_matrix[ys][xs]);
        let threshold = collapser(mono);

        let error = mono - threshold;

        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + ((error * 7) / 16) as i64;
        if xs > 0 { error_matrix[ys+1][xs-1] = &error_matrix[ys+1][xs-1] + ((error * 5) / 16) as i64; }
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + ((error * 3) / 16) as i64;
        error_matrix[ys+1][xs+1] = &error_matrix[ys+1][xs+1] + ((error * 1) / 16) as i64;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

fn jarvis_judice_ninke_mono_dither(image: DynamicImage) -> DynamicImage {
    let collapser = |num: i64| if num < 128 { 0 } else { 255 };

    let mut rgb8_image = image.into_rgb8();
    let (xdim, ydim) = rgb8_image.dimensions();
    let mut error_matrix = vec![vec![0 as i64; (xdim+2) as usize]; (ydim+2) as usize];


    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let mono = average(pixel.channels()) as i64 + *(&error_matrix[ys][xs]);
        let threshold = collapser(mono);

        let error = mono - threshold;

        let (seven48, five48, three48, one48) = (
            ((error * 7) / 48),
            ((error * 5) / 48),
            ((error * 3) / 48),
            ((error * 1) / 48)
        );

        // 1st row
        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + seven48;
        error_matrix[ys][xs+2] = &error_matrix[ys][xs+2] + five48;
        // 1st col
        if xs > 1 {
            error_matrix[ys+1][xs-2] = error_matrix[ys+1][xs-2] + three48;
            error_matrix[ys+2][xs-2] = error_matrix[ys+2][xs-2] + one48;
        }
        // 2nd col
        if xs > 0 {
            error_matrix[ys+1][xs-1] = error_matrix[ys+1][xs-1] + five48;
            error_matrix[ys+2][xs-1] = error_matrix[ys+2][xs-1] + three48;
        }
        // other cols
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + seven48;
        error_matrix[ys+2][xs] = &error_matrix[ys+2][xs] + five48;

        error_matrix[ys+1][xs+1] = &error_matrix[ys+1][xs+1] + five48;
        error_matrix[ys+2][xs+1] = &error_matrix[ys+2][xs+1] + three48;

        error_matrix[ys+1][xs+2] = &error_matrix[ys+1][xs+2] + three48;
        error_matrix[ys+2][xs+2] = &error_matrix[ys+2][xs+2] + one48;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

fn stucki_mono_dither(image: DynamicImage) -> DynamicImage {
    let collapser = |num: i64| if num < 128 { 0 } else { 255 };

    let mut rgb8_image = image.into_rgb8();
    let (xdim, ydim) = rgb8_image.dimensions();
    let mut error_matrix = vec![vec![0 as i64; (xdim+2) as usize]; (ydim+2) as usize];


    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let mono = average(pixel.channels()) as i64 + *(&error_matrix[ys][xs]);
        let threshold = collapser(mono);

        let error = mono - threshold;

        let (eight42, four42, two42, one42) = (
            ((error * 8) / 42),
            ((error * 4) / 42),
            ((error * 2) / 42),
            ((error * 1) / 42)
        );

        // 1st row
        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + eight42;
        error_matrix[ys][xs+2] = &error_matrix[ys][xs+2] + four42;
        // 1st col
        if xs > 1 {
            error_matrix[ys+1][xs-2] = error_matrix[ys+1][xs-2] + two42;
            error_matrix[ys+2][xs-2] = error_matrix[ys+2][xs-2] + one42;
        }
        // 2nd col
        if xs > 0 {
            error_matrix[ys+1][xs-1] = error_matrix[ys+1][xs-1] + four42;
            error_matrix[ys+2][xs-1] = error_matrix[ys+2][xs-1] + two42;
        }
        // other cols
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + eight42;
        error_matrix[ys+2][xs] = &error_matrix[ys+2][xs] + four42;

        error_matrix[ys+1][xs+1] = &error_matrix[ys+1][xs+1] + four42;
        error_matrix[ys+2][xs+1] = &error_matrix[ys+2][xs+1] + two42;

        error_matrix[ys+1][xs+2] = &error_matrix[ys+1][xs+2] + two42;
        error_matrix[ys+2][xs+2] = &error_matrix[ys+2][xs+2] + one42;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}

fn atkinson_mono_dither(image: DynamicImage) -> DynamicImage {
    let collapser = |num: i64| if num < 128 { 0 } else { 255 };

    let mut rgb8_image = image.into_rgb8();
    let (xdim, ydim) = rgb8_image.dimensions();
    let mut error_matrix = vec![vec![0 as i64; (xdim+2) as usize]; (ydim+2) as usize];


    for (x, y, pixel) in rgb8_image.enumerate_pixels_mut() {
        let xs = x as usize;
        let ys = y as usize;

        let mono = average(pixel.channels()) as i64 + *(&error_matrix[ys][xs]);
        let threshold = collapser(mono);

        let error = mono - threshold;
        let error_prop = error / 8;

        error_matrix[ys][xs+1] = &error_matrix[ys][xs+1] + error_prop;
        error_matrix[ys][xs+2] = &error_matrix[ys][xs+2] + error_prop;
        if xs > 0 {
            error_matrix[ys+1][xs-1] = error_matrix[ys+1][xs-1] + error_prop;
        }
        error_matrix[ys+1][xs] = &error_matrix[ys+1][xs] + error_prop;
        error_matrix[ys+1][xs+1] = &error_matrix[ys+1][xs+1] + error_prop;

        error_matrix[ys+2][xs] = &error_matrix[ys+2][xs] + error_prop;

        pixel[0] = threshold as u8;
        pixel[1] = threshold as u8;
        pixel[2] = threshold as u8;
    }

    DynamicImage::ImageRgb8(rgb8_image)
}