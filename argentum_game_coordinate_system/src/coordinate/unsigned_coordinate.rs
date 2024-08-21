use argentum_game_coordinate_system_macros::Coordinate;
use num::{Num, Unsigned};

use super::coordinate_trait::CoordinateTrait;

#[cfg(test)]
mod tests;

#[derive(Coordinate)]
pub struct UnsignedCoordinate<T>
where
    T: Num + Unsigned + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
