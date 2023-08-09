/// Contains all dithering algorithms - in terms of API.
pub mod algorithms;

pub use algorithms::Dither;

/// Contains all the dithering logic for the error propagation based algorithms.
pub mod error;

pub mod basic;
pub mod bayer;
