use std::{fmt::Display, ops::{Add, BitAnd, BitOr, BitXor, Not}};

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
            T::checked_add(&self.x, &rhs.x).unwrap_or_else(panic),
            T::checked_add(&self.x, &rhs.x).unwrap_or_else(panic),
        )
    }
}