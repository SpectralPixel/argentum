// u8: From 0 to 255
// Might need to increase this number if the chunk size grows beyond 255.
type CoordType = u8;

#[derive(PartialEq, Debug)]
pub struct LocalCoord {
    pub x: CoordType,
    pub y: CoordType,
    pub z: CoordType,
}

impl LocalCoord {
    pub const MIN: Self = Self {
        x: CoordType::MIN,
        y: CoordType::MIN,
        z: CoordType::MIN,
    };
    pub const MAX: Self = Self {
        x: CoordType::MAX,
        y: CoordType::MAX,
        z: CoordType::MAX,
    };

    pub fn new(x: CoordType, y: CoordType, z: CoordType) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: CoordType, y: CoordType, z: CoordType) -> bool {
            let result = LocalCoord::new(x, y, z);
            let expected = LocalCoord { x, y, z };
            result == expected
        }
    }

    #[test]
    fn min_pos() {
        let expected = LocalCoord {
            x: CoordType::MIN,
            y: CoordType::MIN,
            z: CoordType::MIN,
        };
        assert_eq!(expected, LocalCoord::MIN);
        assert_eq!(expected.x, CoordType::MIN);
        assert_eq!(expected.y, CoordType::MIN);
        assert_eq!(expected.z, CoordType::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = LocalCoord {
            x: CoordType::MAX,
            y: CoordType::MAX,
            z: CoordType::MAX,
        };
        assert_eq!(expected, LocalCoord::MAX);
        assert_eq!(expected.x, CoordType::MAX);
        assert_eq!(expected.y, CoordType::MAX);
        assert_eq!(expected.z, CoordType::MAX);
    }
}
