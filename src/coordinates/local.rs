// u8: From 0 to 255
// Might need to increase this number if the chunk size grows beyond 255.
type LocalMaxSize = u8;

#[derive(PartialEq, Debug)]
pub struct LocalCoord {
    pub x: LocalMaxSize,
    pub y: LocalMaxSize,
    pub z: LocalMaxSize,
}

impl LocalCoord {
    pub const MIN: Self = Self {
        x: LocalMaxSize::MIN,
        y: LocalMaxSize::MIN,
        z: LocalMaxSize::MIN,
    };
    pub const MAX: Self = Self {
        x: LocalMaxSize::MAX,
        y: LocalMaxSize::MAX,
        z: LocalMaxSize::MAX,
    };

    pub fn new(x: LocalMaxSize, y: LocalMaxSize, z: LocalMaxSize) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: LocalMaxSize, y: LocalMaxSize, z: LocalMaxSize) -> bool {
            let result = LocalCoord::new(x, y, z);
            let expected = LocalCoord { x, y, z };
            result == expected
        }
    }

    #[test]
    fn min_pos() {
        let expected = LocalCoord {
            x: LocalMaxSize::MIN,
            y: LocalMaxSize::MIN,
            z: LocalMaxSize::MIN,
        };
        assert_eq!(expected, LocalCoord::MIN);
        assert_eq!(expected.x, LocalMaxSize::MIN);
        assert_eq!(expected.y, LocalMaxSize::MIN);
        assert_eq!(expected.z, LocalMaxSize::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = LocalCoord {
            x: LocalMaxSize::MAX,
            y: LocalMaxSize::MAX,
            z: LocalMaxSize::MAX,
        };
        assert_eq!(expected, LocalCoord::MAX);
        assert_eq!(expected.x, LocalMaxSize::MAX);
        assert_eq!(expected.y, LocalMaxSize::MAX);
        assert_eq!(expected.z, LocalMaxSize::MAX);
    }
}
