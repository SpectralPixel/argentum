use crate::generate_coordinate_type;

/// `Coordinate`'s field type.
///
/// i32: From −2,147,483,648 to 2,147,483,647.
///
/// I don't believe a larger size is necessary, as the RAM usage per instance
/// would double. Heck, even this is already overkill.
pub type CoordinateType = i32;

generate_coordinate_type!(
    Coordinate,
    CoordinateType,
    "3D Coordinate in absolute space.",
    argentum_game_coordinate_system
);
