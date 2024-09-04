use std::{fmt::Display, ops::{BitAnd, BitOr, BitXor, Not}};

use min_max_traits::{Max, Min};
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer};
use quickcheck::Arbitrary;

#[cfg(test)]
mod tests;

/// `Coord`'s field type.
///
/// i32: From −2,147,483,648 to 2,147,483,647.
///
/// I don't believe a larger size is necessary, as the RAM usage per instance
/// would double. Heck, even this is already overkill.
pub type CoordinateType = i32;

pub type GlobalCoord = Coord<CoordinateType>;

/// 3D Coordinate in absolute space.
#[derive(Clone, Debug, PartialEq)]
pub struct Coord<T>
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

impl<T> Coord<T>
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
    pub const MAX: Self = Self {
        x: T::MAX,
        y: T::MAX,
        z: T::MAX,
    };
    pub const MIN: Self = Self {
        x: T::MIN,
        y: T::MIN,
        z: T::MIN,
    };

    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn splat(n: T) -> Self {
        Self::new(n, n, n)
    }
}

impl<T> Display for Coord<T>
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {}, {}, {})", stringify!(#name), self.x, self.y, self.z)
    }
}


impl<T> Arbitrary for Coord<T>
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
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self::new(
            T::arbitrary(g),
            T::arbitrary(g),
            T::arbitrary(g),
        )
    }
}
