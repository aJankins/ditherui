/// Contains all filtering algorithms.
pub mod algorithms;

pub use algorithms as filters;

use crate::utils::image::{RgbPixelRepr, RgbImageRepr};

pub mod raw;