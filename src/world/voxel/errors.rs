use std::{error::Error, fmt};

use bevy::math::I64Vec3;

#[derive(Debug, Clone)]
pub struct VoxelNotFoundError(pub I64Vec3);

impl fmt::Display for VoxelNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Voxel at {} doesn't exist!", self.0)
    }
}

impl Error for VoxelNotFoundError {}
