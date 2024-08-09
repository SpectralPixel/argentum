use crate::{coordinates::{global, GlobalCoord}, world::World};

// i16: From −32,768 to 32,767
// This should be enough chunks, right?
pub type CoordType = i16;

#[derive(PartialEq, Debug)]
pub struct ChunkCoord {
    pub x: CoordType,
    pub y: CoordType,
    pub z: CoordType,
}

impl ChunkCoord {
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

impl From<GlobalCoord> for ChunkCoord {
    fn from(global_position: GlobalCoord) -> Self {
        fn convert(axis_position: global::CoordType) -> CoordType {
            CoordType::try_from(axis_position / global::CoordType::try_from(World::CHUNK_SIZE).unwrap()).unwrap()
        }

        Self {
            x: convert(global_position.x),
            y: convert(global_position.y),
            z: convert(global_position.z),
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: CoordType, y: CoordType, z: CoordType) -> bool {
            let result = ChunkCoord::new(x, y, z);
            let expected = ChunkCoord { x, y, z };
            result == expected
        }
    }

    #[test]
    fn min_pos() {
        let expected = ChunkCoord {
            x: CoordType::MIN,
            y: CoordType::MIN,
            z: CoordType::MIN,
        };
        assert_eq!(expected, ChunkCoord::MIN);
        assert_eq!(expected.x, CoordType::MIN);
        assert_eq!(expected.y, CoordType::MIN);
        assert_eq!(expected.z, CoordType::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = ChunkCoord {
            x: CoordType::MAX,
            y: CoordType::MAX,
            z: CoordType::MAX,
        };
        assert_eq!(expected, ChunkCoord::MAX);
        assert_eq!(expected.x, CoordType::MAX);
        assert_eq!(expected.y, CoordType::MAX);
        assert_eq!(expected.z, CoordType::MAX);
    }
}
