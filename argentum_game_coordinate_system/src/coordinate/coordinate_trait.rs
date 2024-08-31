use std::{fmt::Display, ops::*};

use quickcheck::Arbitrary;

pub trait CoordinateTrait:
    Sized
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Rem
    + RemAssign
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Not
    + Display
    + Arbitrary
{
    type Type;
    type FieldType;

    const MAX: Self::Type;
    const MIN: Self::Type;

    fn new(x: Self::FieldType, y: Self::FieldType, z: Self::FieldType) -> Self::Type;
    fn splat(n: Self::FieldType) -> Self::Type;
}
