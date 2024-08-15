mod arithmetic;

use crate::coordinate_type;

/// `Coordinate`'s field type.
///
/// i32: From −2,147,483,648 to 2,147,483,647.
///
/// I don't believe a larger size is necessary, as the RAM usage per instance
/// would double. Heck, even this is already overkill.
pub type CoordinateType = i32;

coordinate_type!(
    Coordinate,
    CoordinateType,
    "3D Coordinate in absolute space.",
    argentum_game_coordinate_system
);

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new(x: CoordinateType, y: CoordinateType, z: CoordinateType) -> bool {
            let result = Coordinate::new(x, y, z);
            let expected = Coordinate { x, y, z };
            result == expected
        }
    }

    quickcheck! {
        fn arbitrary(coord: Coordinate) -> bool {
            let x_fine = coord.x >= Coordinate::MIN.x && coord.x <= Coordinate::MAX.x;
            let y_fine = coord.y >= Coordinate::MIN.y && coord.y <= Coordinate::MAX.y;
            let z_fine = coord.z >= Coordinate::MIN.z && coord.z <= Coordinate::MAX.z;
            x_fine && y_fine && z_fine
        }
    }

    #[test]
    fn min_pos() {
        let expected = Coordinate {
            x: CoordinateType::MIN,
            y: CoordinateType::MIN,
            z: CoordinateType::MIN,
        };
        assert_eq!(expected, Coordinate::MIN);
        assert_eq!(expected.x, CoordinateType::MIN);
        assert_eq!(expected.y, CoordinateType::MIN);
        assert_eq!(expected.z, CoordinateType::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = Coordinate {
            x: CoordinateType::MAX,
            y: CoordinateType::MAX,
            z: CoordinateType::MAX,
        };
        assert_eq!(expected, Coordinate::MAX);
        assert_eq!(expected.x, CoordinateType::MAX);
        assert_eq!(expected.y, CoordinateType::MAX);
        assert_eq!(expected.z, CoordinateType::MAX);
    }

    #[test]
    fn display() {
        let pos = Coordinate { x: 1, y: 2, z: 3 };

        assert_eq!(pos.to_string(), "(Coordinate: 1, 2, 3)")
    }

    quickcheck! {
        fn splat(n: CoordinateType) -> bool {
            let coord = Coordinate::splat(n);
            coord.x == coord.y && coord.y == coord.z
        }
    }
}
