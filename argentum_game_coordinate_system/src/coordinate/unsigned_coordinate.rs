use argentum_game_coordinate_system_macros::{Coordinate, CoordinateArithmetic};
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, Unsigned};

use super::coordinate_trait::CoordinateTrait;

#[cfg(test)]
mod tests;

#[derive(Coordinate, CoordinateArithmetic, Clone, Debug, PartialEq)]
pub struct UnsignedCoordinate<T>
where
    T: Integer + Unsigned + Copy + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + Display,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
