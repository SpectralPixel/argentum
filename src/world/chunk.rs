use ndarray::{Array3, Ix3};

use super::cube::Cube;

#[derive(PartialEq, Debug)]
struct Chunk {
    size: usize,
    data: Array3<Cube>,
}

impl Chunk {
    pub fn new(size: usize) -> Self {
        let empty_array: Array3<Cube> = Array3::from_elem(Ix3(size, size, size), Cube::default());
        Chunk {
            size,
            data: empty_array,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_chunk() {
        let size = 2;
        let result = Chunk::new(size);
        let expected = Chunk {
            size,
            data: Array3::from_elem(Ix3(size, size, size), Cube::default()),
        };
        assert_eq!(result, expected);
    }
}
