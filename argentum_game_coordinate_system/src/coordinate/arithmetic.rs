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

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        use crate::CoordinateOutOfBoundsError;
        let panic_if_out_of_bounds = || panic!("{}", CoordinateOutOfBoundsError(self.to_owned()));
        let x = CoordinateType::checked_sub(self.x, rhs.x).unwrap_or_else(panic_if_out_of_bounds);
        let y = CoordinateType::checked_sub(self.y, rhs.y).unwrap_or_else(panic_if_out_of_bounds);
        let z = CoordinateType::checked_sub(self.z, rhs.z).unwrap_or_else(panic_if_out_of_bounds);
        Self::new(x, y, z)
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.to_owned() - rhs;
    }
}

impl Mul for Coordinate {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use crate::CoordinateOutOfBoundsError;
        let panic_if_out_of_bounds = || panic!("{}", CoordinateOutOfBoundsError(self.to_owned()));
        let x = CoordinateType::checked_mul(self.x, rhs.x).unwrap_or_else(panic_if_out_of_bounds);
        let y = CoordinateType::checked_mul(self.y, rhs.y).unwrap_or_else(panic_if_out_of_bounds);
        let z = CoordinateType::checked_mul(self.z, rhs.z).unwrap_or_else(panic_if_out_of_bounds);
        Self::new(x, y, z)
    }
}

impl MulAssign for Coordinate {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.to_owned() * rhs;
    }
}

impl Div for Coordinate {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        use crate::CoordinateOutOfBoundsError;
        let panic_if_out_of_bounds = || panic!("{}", CoordinateOutOfBoundsError(self.to_owned()));
        let x = CoordinateType::checked_div(self.x, rhs.x).unwrap_or_else(panic_if_out_of_bounds);
        let y = CoordinateType::checked_div(self.y, rhs.y).unwrap_or_else(panic_if_out_of_bounds);
        let z = CoordinateType::checked_div(self.z, rhs.z).unwrap_or_else(panic_if_out_of_bounds);
        Self::new(x, y, z)
    }
}

impl DivAssign for Coordinate {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.to_owned() / rhs;
    }
}

impl Neg for Coordinate {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl BitAnd for Coordinate {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x & rhs.x,
            self.y & rhs.y,
            self.z & rhs.z,
        )
    }
}

impl BitAndAssign for Coordinate {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.to_owned() & rhs;
    }
}

impl BitOr for Coordinate {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x | rhs.x,
            self.y | rhs.y,
            self.z | rhs.z,
        )
    }
}

impl BitOrAssign for Coordinate {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.to_owned() | rhs;
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

    #[test]
    fn sub() {
        let result = Coordinate::new(1, 2, 3) - Coordinate::new(5, 0, 10);
        let expected = Coordinate::new(-4, 2, -7);
        assert_eq!(result, expected);
    }

    #[test]
    fn sub_assign() {
        let mut result = Coordinate::new(10, 15, 30);
        result -= Coordinate::new(-5, 10, 23);
        let expected = Coordinate::new(15, 5, 7);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn sub_overflow() {
        let _ = Coordinate::MIN - Coordinate::splat(1);
    }

    #[test]
    fn mul() {
        let result = Coordinate::new(5, 2, 4) * Coordinate::new(1, 0, 2);
        let expected = Coordinate::new(5, 0, 8);
        assert_eq!(result, expected);
    }

    #[test]
    fn mul_assign() {
        let mut result = Coordinate::new(3, 360, 2);
        result *= Coordinate::new(-5, 2, 7);
        let expected = Coordinate::new(-15, 720, 14);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn mul_overflow() {
        let _ = Coordinate::MAX * Coordinate::splat(2);
    }

    #[test]
    fn div() {
        let result = Coordinate::new(5, 2, 4) / Coordinate::new(1, 2, 2);
        let expected = Coordinate::new(5, 1, 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn div_assign() {
        let mut result = Coordinate::new(5, 360, 24);
        result /= Coordinate::new(-5, 2, 4);
        let expected = Coordinate::new(-1, 180, 6);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn div_by_zero() {
        let _ = Coordinate::MAX / Coordinate::splat(0);
    }

    #[test]
    fn neg() {
        let result = -Coordinate::splat(7);
        let expected = Coordinate::splat(-7);
        assert_eq!(result, expected);
    }

    #[test]
    fn and() {
        let coord_a = Coordinate::new(
            0b10101010,
            0b00001111,
            0b10101100,
        );
        let coord_b = Coordinate::splat(0b11001100);
        let result = coord_a & coord_b;
        let expected = Coordinate::new(
            0b10001000,
            0b00001100,
            0b10001100,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn and_assign() {
        let mut coord_a = Coordinate::new(
            0b10101010,
            0b00001111,
            0b10101100,
        );
        let coord_b = Coordinate::splat(0b11001100);
        coord_a &= coord_b;
        let expected = Coordinate::new(
            0b10001000,
            0b00001100,
            0b10001100,
        );
        assert_eq!(coord_a, expected);
    }

    #[test]
    fn or() {
        let coord_a = Coordinate::new(
            0b10101010,
            0b00001111,
            0b10101100,
        );
        let coord_b = Coordinate::splat(0b11001100);
        let result = coord_a | coord_b;
        let expected = Coordinate::new(
            0b11101110,
            0b11001111,
            0b11101100,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn or_assign() {
        let mut coord_a = Coordinate::new(
            0b10101010,
            0b00001111,
            0b10101100,
        );
        let coord_b = Coordinate::splat(0b11001100);
        coord_a |= coord_b;
        let expected = Coordinate::new(
            0b11101110,
            0b11001111,
            0b11101100,
        );
        assert_eq!(coord_a, expected);
    }
}
