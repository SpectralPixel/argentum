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

#[test]
fn add() {
    let result = Coordinate::new(1, 2, 3) + Coordinate::new(5, 0, 10);
    let expected = Coordinate::new(6, 2, 13);
    assert_eq!(result, expected);
}

#[test]
fn add_assign() {
    let mut result = Coordinate::new(10, 15, 30);
    result += Coordinate::new(-5, 10, 23);
    let expected = Coordinate::new(5, 25, 53);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn add_overflow() {
    let _ = Coordinate::MAX + Coordinate::splat(1);
}

#[test]
fn sub() {
    let result = Coordinate::new(1, 2, 3) - Coordinate::new(5, 0, 10);
    let expected = Coordinate::new(-4, 2, -7);
    assert_eq!(result, expected);
}

#[test]
fn sub_assign() {
    let mut result = Coordinate::new(10, 15, 30);
    result -= Coordinate::new(-5, 10, 23);
    let expected = Coordinate::new(15, 5, 7);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn sub_overflow() {
    let _ = Coordinate::MIN - Coordinate::splat(1);
}

#[test]
fn mul() {
    let result = Coordinate::new(5, 2, 4) * Coordinate::new(1, 0, 2);
    let expected = Coordinate::new(5, 0, 8);
    assert_eq!(result, expected);
}

#[test]
fn mul_assign() {
    let mut result = Coordinate::new(3, 360, 2);
    result *= Coordinate::new(-5, 2, 7);
    let expected = Coordinate::new(-15, 720, 14);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn mul_overflow() {
    let _ = Coordinate::MAX * Coordinate::splat(2);
}

#[test]
fn div() {
    let result = Coordinate::new(5, 2, 4) / Coordinate::new(1, 2, 2);
    let expected = Coordinate::new(5, 1, 2);
    assert_eq!(result, expected);
}

#[test]
fn div_assign() {
    let mut result = Coordinate::new(5, 360, 24);
    result /= Coordinate::new(-5, 2, 4);
    let expected = Coordinate::new(-1, 180, 6);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn div_by_zero() {
    let _ = Coordinate::MAX / Coordinate::splat(0);
}

#[test]
fn neg() {
    let result = -Coordinate::splat(7);
    let expected = Coordinate::splat(-7);
    assert_eq!(result, expected);
}

#[test]
fn bit_and() {
    let coord_a = Coordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = Coordinate::splat(0b11001100);
    let result = coord_a & coord_b;
    let expected = Coordinate::new(0b10001000, 0b00001100, 0b10001100);
    assert_eq!(result, expected);
}

#[test]
fn bit_and_assign() {
    let mut coord_a = Coordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = Coordinate::splat(0b11001100);
    coord_a &= coord_b;
    let expected = Coordinate::new(0b10001000, 0b00001100, 0b10001100);
    assert_eq!(coord_a, expected);
}

#[test]
fn bit_or() {
    let coord_a = Coordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = Coordinate::splat(0b11001100);
    let result = coord_a | coord_b;
    let expected = Coordinate::new(0b11101110, 0b11001111, 0b11101100);
    assert_eq!(result, expected);
}

#[test]
fn bit_or_assign() {
    let mut coord_a = Coordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = Coordinate::splat(0b11001100);
    coord_a |= coord_b;
    let expected = Coordinate::new(0b11101110, 0b11001111, 0b11101100);
    assert_eq!(coord_a, expected);
}

#[test]
fn bit_xor() {
    let coord_a = Coordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = Coordinate::splat(0b11001100);
    let result = coord_a ^ coord_b;
    let expected = Coordinate::new(0b01100110, 0b11000011, 0b01100000);
    assert_eq!(result, expected);
}

#[test]
fn bit_xor_assign() {
    let mut coord_a = Coordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = Coordinate::splat(0b11001100);
    coord_a ^= coord_b;
    let expected = Coordinate::new(0b01100110, 0b11000011, 0b01100000);
    assert_eq!(coord_a, expected);
}
