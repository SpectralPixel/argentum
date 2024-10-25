//! # Argentum - Data structures
//!
//! This crate provides wrappers for data structures provided by other crates.
//!
//! Import the `prelude` to gain access to all the crate's types.
//!
//! For more information about Argentum, see the `argentum_game` crate.

/// Import me!
pub mod prelude;

// Each following module corresponds to one crate.
// Do not publish these modules, as that would cause confusion for the end
// user. Please just place all your data structures in the prelude.
mod ndarray;
