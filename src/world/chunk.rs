use ndarray::{Array3, Ix3};

use super::{cube::Cube, World};

#[derive(PartialEq, Debug)]
pub struct Chunk {
    data: Array3<Cube>,
}

impl Chunk {
    pub fn new() -> Self {
        let size = World::CHUNK_SIZE as usize;
        let empty_array: Array3<Cube> = Array3::from_elem(Ix3(size, size, size), Cube::default());
        Chunk { data: empty_array }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_chunk() {
        let size = World::CHUNK_SIZE as usize;
        let result = Chunk::new();
        let expected = Chunk {
            data: Array3::from_elem(Ix3(size, size, size), Cube::default()),
        };
        assert_eq!(result, expected);
    }
}
