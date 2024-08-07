use bevy::{math::Vec3, utils::HashMap};
use chunk::Chunk;

mod chunk;
mod cube;

#[derive(Default)]
struct World {
    data: HashMap<Vec3, Chunk>,
}

impl World {
    pub fn new() -> Self {
        World { data: HashMap::default() }
    }
}
