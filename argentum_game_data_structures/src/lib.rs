//! This crate provides wrappers for data structures provided by dependencies. Each module corresponds to one dependency.

mod prelude;
mod ndarray;

pub use ndarray::VoxelGrid;
