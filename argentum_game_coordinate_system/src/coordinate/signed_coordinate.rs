use argentum_game_coordinate_system_macros::Coordinate;
use num::{Num, Signed};

use super::coordinate_trait::CoordinateTrait;

#[cfg(test)]
mod tests;

#[derive(Coordinate)]
pub struct SignedCoordinate<T>
where
    T: Num + Signed + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
