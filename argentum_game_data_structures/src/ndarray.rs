use std::num::NonZero;

use argentum_game_coordinate_system::prelude::{Coord, RegionSizeType};
use argentum_game_voxel::Voxel;
use ndarray::{Array3, Ix3};

#[derive(Debug)]
pub struct GridCoord(Coord<RegionSizeType>);

impl Into<Ix3> for &GridCoord {
    fn into(self) -> Ix3 {
        Ix3(
            usize::from(self.0.x),
            usize::from(self.0.y),
            usize::from(self.0.z),
        )
    }
}

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

    pub fn get(&self, pos: &GridCoord) -> &Voxel {
        self.get_checked(&pos).unwrap_or_else(|| panic!("The coordinate {:?} is out of bounds! The maximum size for voxel grids is {}.", pos, self.size))
    }

    pub fn get_checked(&self, pos: &GridCoord) -> Option<&Voxel> {
        self.data.get::<Ix3>(pos.into())
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

    #[test]
    fn get() {
        let grid = VoxelGrid::new(NonZero::<u8>::new(1).unwrap());
        let _ = grid.get(&GridCoord(Coord::zero()));
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds() {
        let grid = VoxelGrid::new(NonZero::<u8>::new(1).unwrap());
        let _ = grid.get(&GridCoord(Coord::new(2, 0, 0)));
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds_2() {
        let grid = VoxelGrid::new(NonZero::<u8>::new(1).unwrap());
        let _ = grid.get(&GridCoord(Coord::one()));
    }
}
