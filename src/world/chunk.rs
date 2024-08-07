use bevy::math::I64Vec3;
use ndarray::{Array3, Ix3};

use super::{voxel::Voxel, World};

#[derive(PartialEq, Debug)]
pub struct Chunk {
    data: Array3<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        let size = World::CHUNK_SIZE as usize;
        let empty_array: Array3<Voxel> = Array3::from_elem(Ix3(size, size, size), Voxel::default());
        Chunk { data: empty_array }
    }

    pub fn get_voxel(&self, local_position: &I64Vec3) -> Option<&Voxel> {
        let I64Vec3 { x, y, z } = *local_position;
        let x = usize::try_from(x).unwrap();
        let y = usize::try_from(y).unwrap();
        let z = usize::try_from(z).unwrap();
        self.data.get(Ix3(x, y, z))
    }

    pub fn world_to_chunk_position(world_position: &I64Vec3) -> I64Vec3 {
        let chunk_size = super::World::CHUNK_SIZE as i64;

        I64Vec3::new(
            Self::axis_to_chunk_position(world_position.x, chunk_size),
            Self::axis_to_chunk_position(world_position.y, chunk_size),
            Self::axis_to_chunk_position(world_position.z, chunk_size),
        )
    }

    pub fn world_position_within_chunk(world_position: &I64Vec3) -> I64Vec3 {
        let chunk_size = super::World::CHUNK_SIZE as i64;

        I64Vec3::new(
            Self::wrap_position_into_chunk(world_position.x, chunk_size),
            Self::wrap_position_into_chunk(world_position.y, chunk_size),
            Self::wrap_position_into_chunk(world_position.z, chunk_size),
        )
    }

    fn axis_to_chunk_position(axis_position: i64, chunk_size: i64) -> i64 {
        axis_position / chunk_size
    }

    fn wrap_position_into_chunk(mut axis_position: i64, chunk_size: i64) -> i64 {
        while axis_position < 0 {
            axis_position += chunk_size;
        }

        axis_position % chunk_size
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    #[test]
    fn new_chunk() {
        let size = World::CHUNK_SIZE as usize;
        let result = Chunk::new();
        let expected = Chunk {
            data: Array3::from_elem(Ix3(size, size, size), Voxel::default()),
        };
        assert_eq!(result, expected);
    }

    quickcheck! {
        fn wrapped_position_within_chunk_bounds(random_x: i64, random_y: i64, random_z: i64) -> bool {
            let position = I64Vec3::new(random_x, random_y, random_z);
            let wrapped_position = Chunk::world_position_within_chunk(&position);

            let chunk_size = super::super::World::CHUNK_SIZE as i64;

            let x_is_wrapped = wrapped_position.x >= 0 && wrapped_position.x < chunk_size;
            let y_is_wrapped = wrapped_position.y >= 0 && wrapped_position.y < chunk_size;
            let z_is_wrapped = wrapped_position.z >= 0 && wrapped_position.z < chunk_size;

            x_is_wrapped && y_is_wrapped && z_is_wrapped
        }
    }

    #[test]
    fn axis_to_chunk_position_1() {
        let position = 34;
        let chunk_size = 32;
        let result = Chunk::axis_to_chunk_position(position, chunk_size);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn axis_to_chunk_position_2() {
        let position = 129;
        let chunk_size = 32;
        let result = Chunk::axis_to_chunk_position(position, chunk_size);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn wrap_position_into_chunk_1() {
        let position = 34;
        let chunk_size = 32;
        let result = Chunk::wrap_position_into_chunk(position, chunk_size);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn wrap_position_into_chunk_2() {
        let position = 41;
        let chunk_size = 32;
        let result = Chunk::wrap_position_into_chunk(position, chunk_size);
        let expected = 9;
        assert_eq!(result, expected);
    }
}
