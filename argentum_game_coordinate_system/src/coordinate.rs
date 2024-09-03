use std::{fmt::Display, ops::{BitAnd, BitOr, BitXor, Not}};

use min_max_traits::{Max, Min};
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer};
use quickcheck::Arbitrary;

#[cfg(test)]
mod tests;

/// `Coordinate`'s field type.
///
/// i32: From −2,147,483,648 to 2,147,483,647.
///
/// I don't believe a larger size is necessary, as the RAM usage per instance
/// would double. Heck, even this is already overkill.
pub type CoordinateType = i32;

/// 3D Coordinate in absolute space.
#[derive(Clone, Debug, PartialEq)]
pub struct Coordinate<T>
where
    T: Integer
        + Copy
        + CheckedAdd
        + CheckedSub
        + CheckedMul
        + CheckedDiv
        + Display
        + Max
        + Min
        + Arbitrary
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + Not<Output = T>,
{
    pub x: T,
    pub y: T,
    pub z: T,
}