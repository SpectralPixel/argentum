use core::fmt;

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
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coordinate ({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: CoordinateType, y: CoordinateType, z: CoordinateType) -> bool {
            let result = Coordinate::new(x, y, z);
            let expected = Coordinate { x, y, z };
            result == expected
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

        assert_eq!(pos.to_string(), "Coordinate (1, 2, 3)")
    }
}
