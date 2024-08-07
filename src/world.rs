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

    fn create_chunk(&mut self, chunk_position: &I64Vec3) {
        // Generate chunk data here!
        let new_chunk = Chunk::new();
        self.set_chunk(chunk_position, new_chunk);
    }

    fn set_chunk(&mut self, chunk_position: &I64Vec3, chunk: Chunk) {
        let _ = self.data.insert(*chunk_position, chunk);
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn create_chunk_at(random_x: i64, random_y: i64, random_z: i64) -> bool {
            let mut world = World::new();
            let position = I64Vec3::new(random_x, random_y, random_z);
            world.create_chunk(&position);
            world.data.get(&position).is_some()
        }
    }

    quickcheck! {
        fn set_chunk(random_x: i64, random_y: i64, random_z: i64) -> bool {
            let mut world = World::new();
            let position = I64Vec3::new(random_x, random_y, random_z);
            let new_chunk = Chunk::new();
            world.set_chunk(&position, new_chunk);
            world.data.get(&position).is_some()
        }
    }
}
