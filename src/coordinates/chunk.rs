// i16: From −32,768 to 32,767
// This should be enough chunks, right?
type ChunkMaxSize = i16;

#[derive(PartialEq, Debug)]
pub struct ChunkCoord {
    pub x: ChunkMaxSize,
    pub y: ChunkMaxSize,
    pub z: ChunkMaxSize,
}

impl ChunkCoord {
    pub const MIN: Self = Self {
        x: ChunkMaxSize::MIN,
        y: ChunkMaxSize::MIN,
        z: ChunkMaxSize::MIN,
    };
    pub const MAX: Self = Self {
        x: ChunkMaxSize::MAX,
        y: ChunkMaxSize::MAX,
        z: ChunkMaxSize::MAX,
    };

    pub fn new(x: ChunkMaxSize, y: ChunkMaxSize, z: ChunkMaxSize) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: ChunkMaxSize, y: ChunkMaxSize, z: ChunkMaxSize) -> bool {
            let result = ChunkCoord::new(x, y, z);
            let expected = ChunkCoord { x, y, z };
            result == expected
        }
    }

    #[test]
    fn min_pos() {
        let expected = ChunkCoord {
            x: ChunkMaxSize::MIN,
            y: ChunkMaxSize::MIN,
            z: ChunkMaxSize::MIN,
        };
        assert_eq!(expected, ChunkCoord::MIN);
        assert_eq!(expected.x, ChunkMaxSize::MIN);
        assert_eq!(expected.y, ChunkMaxSize::MIN);
        assert_eq!(expected.z, ChunkMaxSize::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = ChunkCoord {
            x: ChunkMaxSize::MAX,
            y: ChunkMaxSize::MAX,
            z: ChunkMaxSize::MAX,
        };
        assert_eq!(expected, ChunkCoord::MAX);
        assert_eq!(expected.x, ChunkMaxSize::MAX);
        assert_eq!(expected.y, ChunkMaxSize::MAX);
        assert_eq!(expected.z, ChunkMaxSize::MAX);
    }
}
