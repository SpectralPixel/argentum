//! # Argentum - Coordinate System
//!
//! `argentum_game_coordinate_system` contains the coordinate system that is used by
//! Argentum and some utilities related to it.
//!
//! For more information about Argentum, see the `argentum_game` crate.

mod coordinate;
mod macros;
pub mod region;

pub use coordinate::{Coordinate, CoordinateType};
pub use region::Region;
