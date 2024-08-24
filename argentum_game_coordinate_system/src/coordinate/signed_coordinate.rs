use argentum_game_coordinate_system_macros::{Coordinate, CoordinateArithmetic};
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, Signed};

use super::coordinate_trait::CoordinateTrait;

#[cfg(test)]
mod tests;

#[derive(Coordinate, CoordinateArithmetic, Clone, Debug, PartialEq)]
#[signed]
pub struct SignedCoordinate<T>
where
    T: Integer + Signed + Copy + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + Display,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
