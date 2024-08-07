use bevy::{math::I64Vec3, utils::HashMap};
use chunk::Chunk;

mod chunk;
mod voxel;

#[derive(Default)]
struct World {
    data: HashMap<I64Vec3, Chunk>,
}

impl World {
    pub const CHUNK_SIZE: u8 = 32;

    pub fn new() -> Self {
        World {
            data: HashMap::default(),
        }
    }

    fn create_chunk_at(&mut self, position: I64Vec3) {
        // Generate chunk data here!
        let new_chunk = Chunk::new();
        self.set_chunk(position, new_chunk);
    }

    fn set_chunk(&mut self, position: I64Vec3, chunk: Chunk) {
        let _ = self.data.insert(position, chunk);
    }
}
