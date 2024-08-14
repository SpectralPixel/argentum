use crate::{Coordinate, CoordinateType};
use std::ops::*;

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use crate::CoordinateOutOfBoundsError;
        let panic_if_out_of_bounds = || panic!("{}", CoordinateOutOfBoundsError(self.to_owned()));
        let x = CoordinateType::checked_add(self.x, rhs.x).unwrap_or_else(panic_if_out_of_bounds);
        let y = CoordinateType::checked_add(self.y, rhs.y).unwrap_or_else(panic_if_out_of_bounds);
        let z = CoordinateType::checked_add(self.z, rhs.z).unwrap_or_else(panic_if_out_of_bounds);
        Self::new(x, y, z)
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.to_owned() + rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let result = Coordinate::new(1, 2, 3) + Coordinate::new(5, 0, 10);
        let expected = Coordinate::new(6, 2, 13);
        assert_eq!(result, expected);
    }

    #[test]
    fn add_assign() {
        let mut result = Coordinate::new(10, 15, 30);
        result += Coordinate::new(-5, 10, 23);
        let expected = Coordinate::new(5, 25, 53);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn add_overflow() {
        let _ = Coordinate::MAX + Coordinate::splat(1);
    }
}
