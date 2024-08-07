use bevy::{math::Vec3, utils::HashMap};
use chunk::Chunk;

mod chunk;
mod cube;

struct World {
    data: HashMap<Vec3, Chunk>,
}
