use argentum_game_coordinate_system_macros::{Coordinate, CoordinateArithmetic};
use num::{Integer, Unsigned};

use super::coordinate_trait::CoordinateTrait;

#[cfg(test)]
mod tests;

#[derive(Coordinate, CoordinateArithmetic)]
pub struct UnsignedCoordinate<T>
where
    T: Integer + Unsigned + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
