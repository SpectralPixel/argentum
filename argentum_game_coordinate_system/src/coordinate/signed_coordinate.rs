use argentum_game_coordinate_system_macros::Coordinate;
use num::{Num, Signed};

#[cfg(test)]
mod tests;

pub struct SignedCoordinate<T>
where
    T: Num + Signed,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
