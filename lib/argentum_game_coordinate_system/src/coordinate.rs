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

/// Standard coordinate's field type.
///
/// i32: From −2,147,483,648 to 2,147,483,647.
///
/// I don't believe a larger size is necessary, as the RAM usage per instance
/// would double. Heck, even this is already overkill.
pub type CoordinateType = i32;

/// The standard coordinate type for Argentum.
///
/// A 3D Coordinate in absolute space.
pub type Coordinate = Coord<CoordinateType>;

/// A base type to define other coodinate type aliases from.
///
/// To work with the standard coordinate system for Argentum, use the other type.
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
    /// Represents the largest possible coordinate on all axes.
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coord;
    /// assert_eq!(Coord::<i8>::MAX.x, i8::MAX);
    /// assert_eq!(Coord::<i8>::MAX.y, i8::MAX);
    /// assert_eq!(Coord::<i8>::MAX.z, i8::MAX);
    /// ```
    pub const MAX: Self = Self {
        x: T::MAX,
        y: T::MAX,
        z: T::MAX,
    };

    /// Represents the smallest possible coordinate on all axes.
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coord;
    /// assert_eq!(Coord::<i8>::MIN.x, i8::MIN);
    /// assert_eq!(Coord::<i8>::MIN.y, i8::MIN);
    /// assert_eq!(Coord::<i8>::MIN.z, i8::MIN);
    /// ```
    pub const MIN: Self = Self {
        x: T::MIN,
        y: T::MIN,
        z: T::MIN,
    };

    /// Creates a new coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coord;
    /// let pos = Coord::<i8>::new(1, 2, 3);
    /// assert_eq!(pos.x, 1);
    /// assert_eq!(pos.y, 2);
    /// assert_eq!(pos.z, 3);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Creates a new coordinate, assigning all values to the input.
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coord;
    /// let pos = Coord::<i8>::splat(7);
    /// assert_eq!(pos.x, 7);
    /// assert_eq!(pos.y, 7);
    /// assert_eq!(pos.z, 7);
    /// ```
    pub fn splat(n: T) -> Self {
        Self::new(n, n, n)
    }

    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coord;
    /// assert_eq!(Coord::<i8>::zero().x, 0);
    /// assert_eq!(Coord::<i8>::zero().y, 0);
    /// assert_eq!(Coord::<i8>::zero().z, 0);
    pub fn zero() -> Self {
        Self::splat(T::zero())
    }

    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coord;
    /// assert_eq!(Coord::<i8>::unit_x().x, 1);
    /// assert_eq!(Coord::<i8>::unit_x().y, 0);
    /// assert_eq!(Coord::<i8>::unit_x().z, 0);
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coord;
    /// assert_eq!(Coord::<i8>::unit_y().x, 0);
    /// assert_eq!(Coord::<i8>::unit_y().y, 1);
    /// assert_eq!(Coord::<i8>::unit_y().z, 0);
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coord;
    /// assert_eq!(Coord::<i8>::unit_z().x, 0);
    /// assert_eq!(Coord::<i8>::unit_z().y, 0);
    /// assert_eq!(Coord::<i8>::unit_z().z, 1);
    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }

    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::prelude::Coord;
    /// assert_eq!(Coord::<i8>::one().x, 1);
    /// assert_eq!(Coord::<i8>::one().y, 1);
    /// assert_eq!(Coord::<i8>::one().z, 1);
    pub fn one() -> Self {
        Self::splat(T::one())
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
        write!(f, "(Coordinate: {}, {}, {})", self.x, self.y, self.z)
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
