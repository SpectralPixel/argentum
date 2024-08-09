// i32: From −2,147,483,648 to 2,147,483,647
// I don't believe a larger size is necessary, as the RAM usage per instance
// would double. Heck, even this is already overkill.
type GlobalMaxSize = i32;

#[derive(PartialEq, Debug)]
pub struct GlobalCoord {
    pub x: GlobalMaxSize,
    pub y: GlobalMaxSize,
    pub z: GlobalMaxSize,
}

impl GlobalCoord {
    pub const MIN: Self = Self {
        x: GlobalMaxSize::MIN,
        y: GlobalMaxSize::MIN,
        z: GlobalMaxSize::MIN,
    };
    pub const MAX: Self = Self {
        x: GlobalMaxSize::MAX,
        y: GlobalMaxSize::MAX,
        z: GlobalMaxSize::MAX,
    };

    pub fn new(x: GlobalMaxSize, y: GlobalMaxSize, z: GlobalMaxSize) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: GlobalMaxSize, y: GlobalMaxSize, z: GlobalMaxSize) -> bool {
            let result = GlobalCoord::new(x, y, z);
            let expected = GlobalCoord { x, y, z };
            result == expected
        }
    }

    #[test]
    fn min_pos() {
        let expected = GlobalCoord {
            x: GlobalMaxSize::MIN,
            y: GlobalMaxSize::MIN,
            z: GlobalMaxSize::MIN,
        };
        assert_eq!(expected, GlobalCoord::MIN);
        assert_eq!(expected.x, GlobalMaxSize::MIN);
        assert_eq!(expected.y, GlobalMaxSize::MIN);
        assert_eq!(expected.z, GlobalMaxSize::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = GlobalCoord {
            x: GlobalMaxSize::MAX,
            y: GlobalMaxSize::MAX,
            z: GlobalMaxSize::MAX,
        };
        assert_eq!(expected, GlobalCoord::MAX);
        assert_eq!(expected.x, GlobalMaxSize::MAX);
        assert_eq!(expected.y, GlobalMaxSize::MAX);
        assert_eq!(expected.z, GlobalMaxSize::MAX);
    }
}
