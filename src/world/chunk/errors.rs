use std::{error::Error, fmt};

use bevy::math::I64Vec3;

#[derive(Debug, Clone)]
pub struct ChunkNotFoundError(pub I64Vec3);

impl fmt::Display for ChunkNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk at {} doesn't exist!", self.0)
    }
}

impl Error for ChunkNotFoundError {}
