use crate::world::World;

use super::{GlobalCoord, global};

// u8: From 0 to 255
// Might need to increase this number if the chunk size grows beyond 255.
pub type CoordType = u8;

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

impl From<GlobalCoord> for LocalCoord {
    fn from(global_position: GlobalCoord) -> Self {
        fn convert(mut axis_position: global::CoordType) -> CoordType {
            let chunk_size = global::CoordType::try_from(World::CHUNK_SIZE).unwrap();
    
            while axis_position < 0 {
                axis_position += chunk_size;
            }
    
            CoordType::try_from(axis_position % chunk_size).unwrap()
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

    quickcheck! {
        fn wrapped_position_within_chunk_bounds(random_x: global::CoordType, random_y: global::CoordType, random_z: global::CoordType) -> bool {
            let global_position = GlobalCoord::new(random_x, random_y, random_z);
            let local_position = LocalCoord::from(global_position);

            let chunk_size = World::CHUNK_SIZE;

            let x_is_wrapped = local_position.x < chunk_size;
            let y_is_wrapped = local_position.y < chunk_size;
            let z_is_wrapped = local_position.z < chunk_size;

            x_is_wrapped && y_is_wrapped && z_is_wrapped
        }
    }
}
