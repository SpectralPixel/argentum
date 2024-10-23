use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor, Not},
};

use argentum_game_coordinate_system_macros::CoordinateArithmetic;
use min_max_traits::{Max, Min};
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, Signed};
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

pub type Coordinate = Coord<CoordinateType>;

/// 3D Coordinate in absolute space.
#[derive(Clone, Debug, PartialEq, CoordinateArithmetic)]
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
    /// Represents the largest possible `Coordinate` on all axes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use argentum_game_coordinate_system::prelude::{Coordinate, CoordinateType};
    /// assert_eq!(Coordinate::MAX.x, CoordinateType::MAX);
    /// assert_eq!(Coordinate::MAX.y, CoordinateType::MAX);
    /// assert_eq!(Coordinate::MAX.z, CoordinateType::MAX);
    /// ```
    pub const MAX: Self = Self {
        x: T::MAX,
        y: T::MAX,
        z: T::MAX,
    };

    /// Represents the smallest possible `Coordinate` on all axes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use argentum_game_coordinate_system::prelude::{Coordinate, CoordinateType};
    /// assert_eq!(Coordinate::MIN.x, CoordinateType::MIN);
    /// assert_eq!(Coordinate::MIN.y, CoordinateType::MIN);
    /// assert_eq!(Coordinate::MIN.z, CoordinateType::MIN);
    /// ```
    pub const MIN: Self = Self {
        x: T::MIN,
        y: T::MIN,
        z: T::MIN,
    };

    /// Creates a new `Coordinate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coordinate;
    /// let pos = Coordinate::new(1, 2, 3);
    /// assert_eq!(pos.x, 1);
    /// assert_eq!(pos.y, 2);
    /// assert_eq!(pos.z, 3);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Creates a new `Coordinate`, assigning all values to the input.
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coordinate;
    /// let pos = Coordinate::splat(7);
    /// assert_eq!(pos.x, 7);
    /// assert_eq!(pos.y, 7);
    /// assert_eq!(pos.z, 7);
    /// ```
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
        write!(
            f,
            "(Coordinate: {}, {}, {})",
            self.x,
            self.y,
            self.z
        )
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
        Self::new(T::arbitrary(g), T::arbitrary(g), T::arbitrary(g))
    }
}

impl<T> std::ops::Neg for Coord<T>
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
        + Not<Output = T>
        + Signed,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}
