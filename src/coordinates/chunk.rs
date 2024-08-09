// i16: From −32,768 to 32,767
// This should be enough chunks, right?
type ChunkMaxSize = i16;

#[derive(PartialEq, Debug)]
pub struct ChunkCoord {
    pub x: ChunkMaxSize,
    pub y: ChunkMaxSize,
    pub z: ChunkMaxSize,
}

impl ChunkCoord {
    pub fn new(x: ChunkMaxSize, y: ChunkMaxSize, z: ChunkMaxSize) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: ChunkMaxSize, y: ChunkMaxSize, z: ChunkMaxSize) -> bool {
            let result = ChunkCoord::new(x, y, z);
            let expected = ChunkCoord { x, y, z };
            result == expected
        }
    }
}
