use ndarray::{Array3, Ix3};

use super::cube::Cube;

#[derive(PartialEq, Debug)]
pub struct Chunk {
    data: Array3<Cube>,
}

impl Chunk {
    pub fn new(size: u8) -> Self {
        let size = size as usize;
        let empty_array: Array3<Cube> = Array3::from_elem(Ix3(size, size, size), Cube::default());
        Chunk { data: empty_array }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_chunk(size: u8) -> bool {
            let result = Chunk::new(size);
            let size = size as usize;
            let expected = Chunk { data: Array3::from_elem(Ix3(size, size, size), Cube::default()) };
            result == expected
        }
    }
}
