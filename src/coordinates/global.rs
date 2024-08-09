// i32: From −2,147,483,648 to 2,147,483,647
// I don't believe a larger size is necessary, as the RAM usage per instance
// would double. Heck, even this is already overkill.
type CoordType = i32;

#[derive(PartialEq, Debug)]
pub struct GlobalCoord {
    pub x: CoordType,
    pub y: CoordType,
    pub z: CoordType,
}

impl GlobalCoord {
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
            let result = GlobalCoord::new(x, y, z);
            let expected = GlobalCoord { x, y, z };
            result == expected
        }
    }

    #[test]
    fn min_pos() {
        let expected = GlobalCoord {
            x: CoordType::MIN,
            y: CoordType::MIN,
            z: CoordType::MIN,
        };
        assert_eq!(expected, GlobalCoord::MIN);
        assert_eq!(expected.x, CoordType::MIN);
        assert_eq!(expected.y, CoordType::MIN);
        assert_eq!(expected.z, CoordType::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = GlobalCoord {
            x: CoordType::MAX,
            y: CoordType::MAX,
            z: CoordType::MAX,
        };
        assert_eq!(expected, GlobalCoord::MAX);
        assert_eq!(expected.x, CoordType::MAX);
        assert_eq!(expected.y, CoordType::MAX);
        assert_eq!(expected.z, CoordType::MAX);
    }
}
