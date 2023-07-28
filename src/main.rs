mod dither;
mod image;
mod utils;

pub use dither::algorithms;

use crate::image::load_image;

fn main() {
    println!("Hello, world!");

    let mut image = load_image("data/original.png");
    image = dither::algorithms::Algorithms::BasicMono.dither(image);
    image.save("data/basicmono.png").expect("Failed to save image.");
}
