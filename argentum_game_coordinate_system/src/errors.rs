use thiserror::Error;

use crate::prelude::Coord;

/// Useful return type to use when working with regions `Region`s
#[derive(Error, Debug)]
#[error("The coordinate at {0} is out of bounds! The maximum range was {1}.")]
pub struct CoordinateOutOfBoundsError<T, U>(
    /// The coordinate that is supposed to be out of bounds.
    pub Coord<T>,
    /// The maximum range. As regions are meant to be cubed, only an integer has to be provided for this.
    pub U,
)
where
    T: num::Integer
        + Copy
        + num::CheckedAdd
        + num::CheckedSub
        + num::CheckedMul
        + num::CheckedDiv
        + std::fmt::Display
        + min_max_traits::Max
        + min_max_traits::Min
        + quickcheck::Arbitrary
        + core::ops::BitAnd<Output = T>
        + core::ops::BitOr<Output = T>
        + core::ops::BitXor<Output = T>
        + core::ops::Not<Output = T>,
    U: num::Integer;
