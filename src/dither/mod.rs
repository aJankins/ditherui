/// Contains all dithering algorithms - in terms of API.
pub mod algorithms;

/// Contains all the dithering logic for the error propagation based algorithms.
pub mod errorpropagate;

/// Contains some default palettes that can be useful for dithering.
pub mod palettes;

mod basic;
mod bayer;
