use std::{fmt::Display, ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub}};

use min_max_traits::{Max, Min};
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer};
use quickcheck::Arbitrary;

use super::Coord;

impl<T> Add<Self> for Coord<T>
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
    type Output = Coord<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let panic = || panic!("{} cannot be added by {}! This may be caused by integer overflow.", self, rhs);
        Self::new(
            T::checked_add(&self.x, &rhs.x).unwrap_or_else(panic),
            T::checked_add(&self.y, &rhs.y).unwrap_or_else(panic),
            T::checked_add(&self.z, &rhs.z).unwrap_or_else(panic),
        )
    }
}

impl<T> Add<T> for Coord<T>
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
    type Output = Coord<T>;
    fn add(self, rhs: T) -> Self::Output {
        let panic = || panic!("{} cannot be added by {}! This may be caused by integer overflow.", self, rhs);
        Self::new(
            T::checked_add(&self.x, &rhs).unwrap_or_else(panic),
            T::checked_add(&self.y, &rhs).unwrap_or_else(panic),
            T::checked_add(&self.z, &rhs).unwrap_or_else(panic),
        )
    }
}

impl<T> Sub<Self> for Coord<T>
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
    type Output = Coord<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let panic = || panic!("{} cannot be subtracted by {}! This may be caused by integer overflow.", self, rhs);
        Self::new(
            T::checked_sub(&self.x, &rhs.x).unwrap_or_else(panic),
            T::checked_sub(&self.y, &rhs.y).unwrap_or_else(panic),
            T::checked_sub(&self.z, &rhs.z).unwrap_or_else(panic),
        )
    }
}

impl<T> Sub<T> for Coord<T>
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
    type Output = Coord<T>;
    fn sub(self, rhs: T) -> Self::Output {
        let panic = || panic!("{} cannot be subtracted by {}! This may be caused by integer overflow.", self, rhs);
        Self::new(
            T::checked_sub(&self.x, &rhs).unwrap_or_else(panic),
            T::checked_sub(&self.y, &rhs).unwrap_or_else(panic),
            T::checked_sub(&self.z, &rhs).unwrap_or_else(panic),
        )
    }
}

impl<T> Mul<Self> for Coord<T>
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
    type Output = Coord<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let panic = || panic!("{} cannot be multiplied by {}! This may be caused by integer overflow.", self, rhs);
        Self::new(
            T::checked_mul(&self.x, &rhs.x).unwrap_or_else(panic),
            T::checked_mul(&self.y, &rhs.y).unwrap_or_else(panic),
            T::checked_mul(&self.z, &rhs.z).unwrap_or_else(panic),
        )
    }
}

impl<T> Mul<T> for Coord<T>
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
    type Output = Coord<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let panic = || panic!("{} cannot be multiplied by {}! This may be caused by integer overflow.", self, rhs);
        Self::new(
            T::checked_mul(&self.x, &rhs).unwrap_or_else(panic),
            T::checked_mul(&self.y, &rhs).unwrap_or_else(panic),
            T::checked_mul(&self.z, &rhs).unwrap_or_else(panic),
        )
    }
}
