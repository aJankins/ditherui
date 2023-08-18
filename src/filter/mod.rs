/// Contains all filtering algorithms.
pub mod algorithms;

pub use algorithms as filters;

/// Raw implementations of the filters, done for organization.
/// 
/// There's both specific implementations, and more generic implementations using `From` and `Into`.
pub mod raw;