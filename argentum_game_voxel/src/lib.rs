//! Argentum - Voxel
//! 
//! `argentum_game_voxel` contains the voxel type used for Argentum.
//! For more information about Argentum, see the `argentum_game` crate.

type VoxelIdType = u8;

/// The main building block of voxel worlds.
/// Contains data on what kind of material the voxel should represent.
#[derive(PartialEq, Debug, Default, Clone)]
pub struct Voxel {
    id: VoxelIdType
}

impl Voxel {
    /// Creates a new Voxel
    /// 
    /// # Examples
    /// 
    /// ```
    /// let voxel = argentum_game_voxel::Voxel::new(7);
    /// 
    /// ```
    pub fn new(id: VoxelIdType) -> Self {
        Voxel { id }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_cube(id: VoxelIdType) -> bool {
            let result = Voxel::new(id);
            let expected = Voxel { id };
            result == expected
        }
    }
}
