mod arithmetic;

/// `Coordinate`'s field type.
///
/// i32: From −2,147,483,648 to 2,147,483,647.
///
/// I don't believe a larger size is necessary, as the RAM usage per instance
/// would double. Heck, even this is already overkill.
pub type CoordinateType = i32;

/// 3D Coordinate in absolute space.
#[derive(PartialEq, Debug, Clone)]
pub struct Coordinate {
    pub x: CoordinateType,
    pub y: CoordinateType,
    pub z: CoordinateType,
}

impl Coordinate {
    /// Represents the smallest possible coordinate on all axes.
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::{CoordinateType, Coordinate};
    /// assert_eq!(Coordinate::MIN.x, CoordinateType::MIN);
    /// assert_eq!(Coordinate::MIN.y, CoordinateType::MIN);
    /// assert_eq!(Coordinate::MIN.z, CoordinateType::MIN);
    /// ```
    pub const MIN: Self = Self {
        x: CoordinateType::MIN,
        y: CoordinateType::MIN,
        z: CoordinateType::MIN,
    };

    /// Represents the largest possible coordinate on all axes.
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::{CoordinateType, Coordinate};
    /// assert_eq!(Coordinate::MAX.x, CoordinateType::MAX);
    /// assert_eq!(Coordinate::MAX.y, CoordinateType::MAX);
    /// assert_eq!(Coordinate::MAX.z, CoordinateType::MAX);
    /// ```
    pub const MAX: Self = Self {
        x: CoordinateType::MAX,
        y: CoordinateType::MAX,
        z: CoordinateType::MAX,
    };

    /// Creates a new Coordinate
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::Coordinate;
    /// let pos = Coordinate::new(1, 2, 3);
    /// assert_eq!(pos.x, 1);
    /// assert_eq!(pos.y, 2);
    /// assert_eq!(pos.z, 3);
    /// ```
    pub fn new(x: CoordinateType, y: CoordinateType, z: CoordinateType) -> Self {
        Self { x, y, z }
    }

    /// Creates a new coordinate, assigning all values to the input.
    ///
    /// # Examples
    ///
    /// ```
    /// use argentum_game_coordinate_system::Coordinate;
    /// let pos = Coordinate::splat(7);
    /// assert_eq!(pos.x, 7);
    /// assert_eq!(pos.y, 7);
    /// assert_eq!(pos.z, 7);
    /// ```
    pub fn splat(n: CoordinateType) -> Self {
        Self::new(n, n, n)
    }
}

impl core::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl quickcheck::Arbitrary for Coordinate {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self::new(
            CoordinateType::arbitrary(g),
            CoordinateType::arbitrary(g),
            CoordinateType::arbitrary(g),
        )
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new(x: CoordinateType, y: CoordinateType, z: CoordinateType) -> bool {
            let result = Coordinate::new(x, y, z);
            let expected = Coordinate { x, y, z };
            result == expected
        }
    }

    quickcheck! {
        fn arbitrary(coord: Coordinate) -> bool {
            let x_fine = coord.x >= Coordinate::MIN.x && coord.x <= Coordinate::MAX.x;
            let y_fine = coord.y >= Coordinate::MIN.y && coord.y <= Coordinate::MAX.y;
            let z_fine = coord.z >= Coordinate::MIN.z && coord.z <= Coordinate::MAX.z;
            x_fine && y_fine && z_fine
        }
    }

    #[test]
    fn min_pos() {
        let expected = Coordinate {
            x: CoordinateType::MIN,
            y: CoordinateType::MIN,
            z: CoordinateType::MIN,
        };
        assert_eq!(expected, Coordinate::MIN);
        assert_eq!(expected.x, CoordinateType::MIN);
        assert_eq!(expected.y, CoordinateType::MIN);
        assert_eq!(expected.z, CoordinateType::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = Coordinate {
            x: CoordinateType::MAX,
            y: CoordinateType::MAX,
            z: CoordinateType::MAX,
        };
        assert_eq!(expected, Coordinate::MAX);
        assert_eq!(expected.x, CoordinateType::MAX);
        assert_eq!(expected.y, CoordinateType::MAX);
        assert_eq!(expected.z, CoordinateType::MAX);
    }

    #[test]
    fn display() {
        let pos = Coordinate { x: 1, y: 2, z: 3 };

        assert_eq!(pos.to_string(), "(1, 2, 3)")
    }

    quickcheck! {
        fn splat(n: CoordinateType) -> bool {
            let coord = Coordinate::splat(n);
            coord.x == coord.y && coord.y == coord.z
        }
    }
}
