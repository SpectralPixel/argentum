use std::num::NonZero;

use argentum_game_coordinate_system::region::SizeType;
use argentum_game_voxel::Voxel;
use ndarray::{Array3, Ix3};

pub struct VoxelGrid {
    size: SizeType,
    data: Array3<Voxel>
}

impl VoxelGrid {
    pub fn new(size: NonZero<SizeType>) -> Self {
        let size = size.get();
        let s = usize::from(size);
        Self {
            size,
            data: Array3::from_elem(Ix3(s, s, s), Voxel::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new(size: NonZero<SizeType>) -> bool {
            let _ = VoxelGrid::new(size);
            true
        }
    }
}
