use std::num::NonZero;

use argentum_game_coordinate_system::prelude::{Coord, RegionSizeType};
use argentum_game_voxel::Voxel;
use ndarray::{Array3, Ix3};

pub type GridCoord = Coord<RegionSizeType>;

pub struct GridCoordConverter(GridCoord);

impl From<&GridCoord> for GridCoordConverter {
    fn from(val: &GridCoord) -> Self {
        GridCoordConverter(Coord::new(val.x, val.y, val.z))
    }
}

impl From<GridCoordConverter> for Ix3 {
    fn from(val: GridCoordConverter) -> Self {
        Ix3(
            usize::from(val.0.x),
            usize::from(val.0.y),
            usize::from(val.0.z),
        )
    }
}

fn convert(coord: &GridCoord) -> Ix3 {
    let intermediary_step: GridCoordConverter = coord.into();
    intermediary_step.into()
}

pub struct VoxelGrid {
    size: RegionSizeType,
    data: Array3<Voxel>,
}

impl VoxelGrid {
    pub fn new(size: NonZero<RegionSizeType>) -> Self {
        let size = size.get();
        let dims = GridCoord::splat(size);
        let dims = convert(&dims);
        Self {
            size,
            data: Array3::from_elem(dims, Voxel::default()),
        }
    }

    pub fn get(&self, pos: &GridCoord) -> &Voxel {
        self.get_checked(pos).unwrap_or_else(|| {
            panic!(
                "The coordinate {:?} is out of bounds! The maximum size for voxel grids is {}.",
                pos, self.size
            )
        })
    }

    pub fn get_checked(&self, pos: &GridCoord) -> Option<&Voxel> {
        self.data.get(convert(pos))
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
        let _ = grid.get(&GridCoord::zero());
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds() {
        let grid = VoxelGrid::new(NonZero::<u8>::new(1).unwrap());
        let _ = grid.get(&GridCoord::new(2, 0, 0));
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds_2() {
        let grid = VoxelGrid::new(NonZero::<u8>::new(1).unwrap());
        let _ = grid.get(&GridCoord::one());
    }
}
