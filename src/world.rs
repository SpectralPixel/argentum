use bevy::{math::I64Vec3, utils::HashMap};
use chunk::Chunk;

mod chunk;
mod cube;

#[derive(Default)]
struct World {
    data: HashMap<I64Vec3, Chunk>,
}

impl World {
    pub const CHUNK_SIZE: u8 = 32;

    pub fn new() -> Self {
        World { data: HashMap::default() }
    }
}
