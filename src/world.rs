use bevy::{math::I64Vec3, utils::HashMap};
use chunk::Chunk;

mod chunk;
mod cube;

#[derive(Default)]
struct World {
    data: HashMap<I64Vec3, Chunk>,
}

impl World {
    pub fn new() -> Self {
        World { data: HashMap::default() }
    }
}
