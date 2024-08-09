use std::error::Error;

use bevy::math::I64Vec3;
use ndarray::{Array3, Ix3};

use super::{
    voxel::{Voxel, VoxelNotFoundError, WrappedPositionOutOfBoundsError},
    World,
};

pub mod errors;

pub use errors::*;

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

    pub fn get_voxel(&self, local_position: &I64Vec3) -> Result<&Voxel, Box<dyn Error>> {
        let I64Vec3 { x, y, z } = *local_position;
        let x = usize::try_from(x).unwrap();
        let y = usize::try_from(y).unwrap();
        let z = usize::try_from(z).unwrap();
        match self.data.get(Ix3(x, y, z)) {
            Some(voxel_reference) => Ok(voxel_reference),
            None => Err(Box::new(VoxelNotFoundError(local_position.clone()))),
        }
    }

    pub fn set_voxel(
        &mut self,
        local_position: &I64Vec3,
        voxel: Voxel,
    ) -> Result<(), Box<dyn Error>> {
        let I64Vec3 { x, y, z } = *local_position;
        let x = usize::try_from(x)?;
        let y = usize::try_from(y)?;
        let z = usize::try_from(z)?;
        let current_voxel = self
            .data
            .get_mut(Ix3(x, y, z))
            .ok_or(WrappedPositionOutOfBoundsError(local_position.clone()))?;
        *current_voxel = voxel;
        Ok(())
    }

    pub fn global_to_chunk_coord(global_position: &I64Vec3) -> I64Vec3 {
        let chunk_size = super::World::CHUNK_SIZE as i64;

        I64Vec3::new(
            Self::global_to_chunk_axis(global_position.x, chunk_size),
            Self::global_to_chunk_axis(global_position.y, chunk_size),
            Self::global_to_chunk_axis(global_position.z, chunk_size),
        )
    }

    pub fn global_to_local_coord(global_position: &I64Vec3) -> I64Vec3 {
        let chunk_size = super::World::CHUNK_SIZE as i64;

        I64Vec3::new(
            Self::global_to_local_axis(global_position.x, chunk_size),
            Self::global_to_local_axis(global_position.y, chunk_size),
            Self::global_to_local_axis(global_position.z, chunk_size),
        )
    }

    fn global_to_chunk_axis(axis_position: i64, chunk_size: i64) -> i64 {
        axis_position / chunk_size
    }

    fn global_to_local_axis(mut axis_position: i64, chunk_size: i64) -> i64 {
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
}
