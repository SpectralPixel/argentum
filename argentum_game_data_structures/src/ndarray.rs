use std::num::NonZero;

use argentum_game_coordinate_system::prelude::{Coord, RegionSizeType};
use argentum_game_voxel::Voxel;
use ndarray::{Array3, Ix3};

pub struct GridCoord(Coord<RegionSizeType>);

pub struct VoxelGrid {
    size: RegionSizeType,
    data: Array3<Voxel>,
}

impl VoxelGrid {
    pub fn new(size: NonZero<RegionSizeType>) -> Self {
        let size = size.get();
        let s = usize::from(size);
        Self {
            size,
            data: Array3::from_elem(Ix3(s, s, s), Voxel::default()),
        }
    }

    pub fn get(&self, pos: GridCoord) -> Option<Voxel> {
        None
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new(size: NonZero<RegionSizeType>) -> bool {
            let _ = VoxelGrid::new(size);
            true
        }
    }
}
