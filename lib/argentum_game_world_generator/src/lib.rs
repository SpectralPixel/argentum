// use argentum_game_coordinate_system::{Coordinate, CoordinateType, Region};
// use argentum_game_voxel::Voxel;
// use ndarray::{Array3, Ix3};

// pub fn generate(position: Coordinate, size: CoordinateType) -> Array3<Voxel> {
//     let region = Region::new(position, size);
//     let size = size as usize;
//     let mut chunk_data = Array3::from_elem(Ix3(size, size, size), Voxel::default());

//     for position in region {
//         match position.y {
//             _ => {
//                 let voxel = chunk_data.get_mut(Ix3(position.x, position.y, position.z));
//                 *voxel = Voxel::new(0);
//             },
//         }
//     }

//     chunk_data
// }
