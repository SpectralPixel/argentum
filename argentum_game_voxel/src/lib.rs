//! # Argentum - Voxel
//! 
//! `argentum_game_voxel` contains the voxel type used in Argentum.
//! 
//! For more information about Argentum, see the `argentum_game` crate.

type VoxelIdType = u8;

/// The main building block of voxel worlds.
/// 
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
    /// use argentum_game_voxel::Voxel;
    /// let voxel = Voxel::new(7);
    /// ```
    pub fn new(id: VoxelIdType) -> Self {
        Voxel { id }
    }

    /// Return a Voxel's id
    /// 
    /// # Examples
    /// 
    /// ```
    /// use argentum_game_voxel::Voxel;
    /// let voxel = Voxel::new(7);
    /// let result = voxel.id();
    /// assert_eq!(result, 7);
    /// ```
    pub fn id(&self) -> VoxelIdType {
        self.id
    }    
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new(id: VoxelIdType) -> bool {
            let result = Voxel::new(id);
            let expected = Voxel { id };
            result == expected
        }
    }

    quickcheck! {
        fn id(id: VoxelIdType) -> bool {
            let voxel = Voxel::new(id);
            let result = voxel.id();
            result == id
        }
    }
}
