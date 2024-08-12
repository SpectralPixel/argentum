//! # Argentum - Coordinate System
//!
//! `argentum_game_voxel` contains the coordinate system that is used by
//! Argentum and some utilities related to it.
//!
//! For more information about Argentum, see the `argentum_game` crate.

use core::fmt;

/// `GlobalCoord`'s field type.
/// 
/// i32: From −2,147,483,648 to 2,147,483,647.
/// 
/// I don't believe a larger size is necessary, as the RAM usage per instance
/// would double. Heck, even this is already overkill.
pub type GlobalCoordType = i32;

/// 3D Coordinate in absolute space.
#[derive(PartialEq, Debug, Clone)]
pub struct GlobalCoord {
    pub x: GlobalCoordType,
    pub y: GlobalCoordType,
    pub z: GlobalCoordType,
}

impl GlobalCoord {
    /// Represents the smallest possible coordinate on all axes.
    /// 
    /// All axes are currently set to `GlobalCoordType::MIN`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use argentum_game_coordinate_system::{GlobalCoordType, GlobalCoord};
    /// assert_eq!(GlobalCoord::MIN.x, GlobalCoordType::MIN);
    /// ```
    pub const MIN: Self = Self {
        x: GlobalCoordType::MIN,
        y: GlobalCoordType::MIN,
        z: GlobalCoordType::MIN,
    };

    /// Represents the largest possible coordinate on all axes.
    /// 
    /// All axes are currently set to `GlobalCoordType::MAX`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use argentum_game_coordinate_system::{GlobalCoordType, GlobalCoord};
    /// assert_eq!(GlobalCoord::MAX.x, GlobalCoordType::MAX);
    /// ```
    pub const MAX: Self = Self {
        x: GlobalCoordType::MAX,
        y: GlobalCoordType::MAX,
        z: GlobalCoordType::MAX,
    };

    pub fn new(x: GlobalCoordType, y: GlobalCoordType, z: GlobalCoordType) -> Self {
        Self { x, y, z }
    }
}

impl fmt::Display for GlobalCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GlobalCoord ({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: GlobalCoordType, y: GlobalCoordType, z: GlobalCoordType) -> bool {
            let result = GlobalCoord::new(x, y, z);
            let expected = GlobalCoord { x, y, z };
            result == expected
        }
    }

    #[test]
    fn min_pos() {
        let expected = GlobalCoord {
            x: GlobalCoordType::MIN,
            y: GlobalCoordType::MIN,
            z: GlobalCoordType::MIN,
        };
        assert_eq!(expected, GlobalCoord::MIN);
        assert_eq!(expected.x, GlobalCoordType::MIN);
        assert_eq!(expected.y, GlobalCoordType::MIN);
        assert_eq!(expected.z, GlobalCoordType::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = GlobalCoord {
            x: GlobalCoordType::MAX,
            y: GlobalCoordType::MAX,
            z: GlobalCoordType::MAX,
        };
        assert_eq!(expected, GlobalCoord::MAX);
        assert_eq!(expected.x, GlobalCoordType::MAX);
        assert_eq!(expected.y, GlobalCoordType::MAX);
        assert_eq!(expected.z, GlobalCoordType::MAX);
    }

    #[test]
    fn display() {
        let pos = GlobalCoord { x: 1, y: 2, z: 3 };

        assert_eq!(pos.to_string(), "GlobalCoord (1, 2, 3)")
    }
}
