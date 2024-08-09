pub mod errors;

pub use errors::*;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Voxel {
    index: u8,
}

impl Voxel {
    pub fn new(index: u8) -> Self {
        Voxel { index }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_cube(index: u8) -> bool {
            let result = Voxel::new(index);
            let expected = Voxel { index };
            result == expected
        }
    }
}
