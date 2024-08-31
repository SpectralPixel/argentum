use quickcheck::quickcheck;

use super::*;

quickcheck! {
    fn new(x: usize, y: usize, z: usize) -> bool {
        let result = UnsignedCoordinate::new(x, y, z);
        let expected = UnsignedCoordinate { x, y, z };
        result == expected
    }
}

quickcheck! {
    fn arbitrary(coord: UnsignedCoordinate<usize>) -> bool {
        let x_fine = coord.x >= UnsignedCoordinate::MIN.x && coord.x <= UnsignedCoordinate::MAX.x;
        let y_fine = coord.y >= UnsignedCoordinate::MIN.y && coord.y <= UnsignedCoordinate::MAX.y;
        let z_fine = coord.z >= UnsignedCoordinate::MIN.z && coord.z <= UnsignedCoordinate::MAX.z;
        x_fine && y_fine && z_fine
    }
}

#[test]
fn min_pos() {
    let expected = UnsignedCoordinate {
        x: usize::MIN,
        y: usize::MIN,
        z: usize::MIN,
    };
    assert_eq!(expected, UnsignedCoordinate::MIN);
    assert_eq!(expected.x, usize::MIN);
    assert_eq!(expected.y, usize::MIN);
    assert_eq!(expected.z, usize::MIN);
}

#[test]
fn max_pos() {
    let expected = UnsignedCoordinate {
        x: usize::MAX,
        y: usize::MAX,
        z: usize::MAX,
    };
    assert_eq!(expected, UnsignedCoordinate::MAX);
    assert_eq!(expected.x, usize::MAX);
    assert_eq!(expected.y, usize::MAX);
    assert_eq!(expected.z, usize::MAX);
}

#[test]
fn display() {
    let pos = UnsignedCoordinate::<usize> { x: 1, y: 2, z: 3 };

    assert_eq!(pos.to_string(), "(UnsignedCoordinate: 1, 2, 3)")
}

quickcheck! {
    fn splat(n: usize) -> bool {
        let coord = UnsignedCoordinate::splat(n);
        coord.x == coord.y && coord.y == coord.z
    }
}

#[test]
fn add() {
    let result = UnsignedCoordinate::<usize>::new(1, 2, 3) + UnsignedCoordinate::new(5, 0, 10);
    let expected = UnsignedCoordinate::new(6, 2, 13);
    assert_eq!(result, expected);
}

#[test]
fn add_single() {
    let result = UnsignedCoordinate::<usize>::new(1, 2, 3) + 10;
    let expected = UnsignedCoordinate::new(11, 12, 13);
    assert_eq!(result, expected);
}

#[test]
fn add_assign() {
    let mut result = UnsignedCoordinate::<usize>::new(10, 15, 30);
    result += UnsignedCoordinate::new(5, 10, 23);
    let expected = UnsignedCoordinate::new(15, 25, 53);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn add_overflow() {
    let _ = UnsignedCoordinate::<usize>::MAX + UnsignedCoordinate::splat(1);
}

#[test]
#[should_panic]
fn add_overflow_single() {
    let _ = UnsignedCoordinate::<usize>::MAX + 1;
}

#[test]
fn sub() {
    let result = UnsignedCoordinate::<usize>::new(5, 2, 10) - UnsignedCoordinate::new(1, 0, 3);
    let expected = UnsignedCoordinate::new(4, 2, 7);
    assert_eq!(result, expected);
}

#[test]
fn sub_single() {
    let result = UnsignedCoordinate::<usize>::new(5, 2, 10) - 2;
    let expected = UnsignedCoordinate::new(3, 0, 8);
    assert_eq!(result, expected);
}

#[test]
fn sub_assign() {
    let mut result = UnsignedCoordinate::<usize>::new(10, 15, 30);
    result -= UnsignedCoordinate::new(5, 10, 23);
    let expected = UnsignedCoordinate::new(5, 5, 7);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn sub_overflow() {
    let _ = UnsignedCoordinate::<usize>::MIN - UnsignedCoordinate::splat(1);
}

#[test]
#[should_panic]
fn sub_overflow_single() {
    let _ = UnsignedCoordinate::<usize>::MIN - 1;
}

#[test]
fn mul() {
    let result = UnsignedCoordinate::<usize>::new(5, 2, 4) * UnsignedCoordinate::new(1, 0, 2);
    let expected = UnsignedCoordinate::new(5, 0, 8);
    assert_eq!(result, expected);
}

#[test]
fn mul_single() {
    let result = UnsignedCoordinate::<usize>::new(5, 2, 4) * 0;
    let expected = UnsignedCoordinate::new(0, 0, 0);
    assert_eq!(result, expected);
}

#[test]
fn mul_assign() {
    let mut result = UnsignedCoordinate::<usize>::new(3, 360, 2);
    result *= UnsignedCoordinate::new(5, 2, 7);
    let expected = UnsignedCoordinate::new(15, 720, 14);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn mul_overflow() {
    let _ = UnsignedCoordinate::<usize>::MAX * UnsignedCoordinate::splat(2);
}

#[test]
#[should_panic]
fn mul_overflow_single() {
    let _ = UnsignedCoordinate::<usize>::MAX * 2;
}

#[test]
fn div() {
    let result = UnsignedCoordinate::<usize>::new(5, 2, 4) / UnsignedCoordinate::new(1, 2, 2);
    let expected = UnsignedCoordinate::new(5, 1, 2);
    assert_eq!(result, expected);
}

#[test]
fn div_single() {
    let result = UnsignedCoordinate::<usize>::new(9, 27, 3) / 3;
    let expected = UnsignedCoordinate::new(3, 9, 1);
    assert_eq!(result, expected);
}

#[test]
fn div_assign() {
    let mut result = UnsignedCoordinate::<usize>::new(5, 360, 24);
    result /= UnsignedCoordinate::new(5, 2, 4);
    let expected = UnsignedCoordinate::new(1, 180, 6);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn div_by_zero() {
    let _ = UnsignedCoordinate::<usize>::MAX / UnsignedCoordinate::splat(0);
}

#[test]
#[should_panic]
fn div_by_zero_single() {
    let _ = UnsignedCoordinate::<usize>::MAX / 0;
}

#[test]
fn rem() {
    let result = UnsignedCoordinate::<usize>::new(7, 12, 4) % UnsignedCoordinate::new(5, 7, 2);
    let expected = UnsignedCoordinate::new(2, 5, 0);
    assert_eq!(result, expected);
}

#[test]
fn rem_single() {
    let result = UnsignedCoordinate::<usize>::new(10, 12, 4) % 5;
    let expected = UnsignedCoordinate::new(0, 2, 4);
    assert_eq!(result, expected);
}

#[test]
fn rem_single_assign() {
    let mut result = UnsignedCoordinate::<usize>::new(7, 11, 38);
    result %= 6;
    let expected = UnsignedCoordinate::new(1, 5, 2);
    assert_eq!(result, expected);
}

#[test]
fn not() {
    let result = !(UnsignedCoordinate::<usize>::MAX - UnsignedCoordinate::splat(5));
    let expected = UnsignedCoordinate::MIN + UnsignedCoordinate::splat(5);
    assert_eq!(result, expected);
}

#[test]
fn bit_and() {
    let coord_a = UnsignedCoordinate::<usize>::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = UnsignedCoordinate::splat(0b11001100);
    let result = coord_a & coord_b;
    let expected = UnsignedCoordinate::new(0b10001000, 0b00001100, 0b10001100);
    assert_eq!(result, expected);
}

#[test]
fn bit_and_single() {
    let coord_a = UnsignedCoordinate::<usize>::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = 0b11001100;
    let result = coord_a & coord_b;
    let expected = UnsignedCoordinate::new(0b10001000, 0b00001100, 0b10001100);
    assert_eq!(result, expected);
}

#[test]
fn bit_and_assign() {
    let mut coord_a = UnsignedCoordinate::<usize>::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = UnsignedCoordinate::splat(0b11001100);
    coord_a &= coord_b;
    let expected = UnsignedCoordinate::new(0b10001000, 0b00001100, 0b10001100);
    assert_eq!(coord_a, expected);
}

#[test]
fn bit_or() {
    let coord_a = UnsignedCoordinate::<usize>::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = UnsignedCoordinate::splat(0b11001100);
    let result = coord_a | coord_b;
    let expected = UnsignedCoordinate::new(0b11101110, 0b11001111, 0b11101100);
    assert_eq!(result, expected);
}

#[test]
fn bit_or_single() {
    let coord_a = UnsignedCoordinate::<usize>::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = 0b11001100;
    let result = coord_a | coord_b;
    let expected = UnsignedCoordinate::new(0b11101110, 0b11001111, 0b11101100);
    assert_eq!(result, expected);
}

#[test]
fn bit_or_assign() {
    let mut coord_a = UnsignedCoordinate::<usize>::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = UnsignedCoordinate::splat(0b11001100);
    coord_a |= coord_b;
    let expected = UnsignedCoordinate::new(0b11101110, 0b11001111, 0b11101100);
    assert_eq!(coord_a, expected);
}

#[test]
fn bit_xor() {
    let coord_a = UnsignedCoordinate::<usize>::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = UnsignedCoordinate::splat(0b11001100);
    let result = coord_a ^ coord_b;
    let expected = UnsignedCoordinate::new(0b01100110, 0b11000011, 0b01100000);
    assert_eq!(result, expected);
}

#[test]
fn bit_xor_single() {
    let coord_a = UnsignedCoordinate::<usize>::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = 0b11001100;
    let result = coord_a ^ coord_b;
    let expected = UnsignedCoordinate::new(0b01100110, 0b11000011, 0b01100000);
    assert_eq!(result, expected);
}

#[test]
fn bit_xor_assign() {
    let mut coord_a = UnsignedCoordinate::<usize>::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = UnsignedCoordinate::splat(0b11001100);
    coord_a ^= coord_b;
    let expected = UnsignedCoordinate::new(0b01100110, 0b11000011, 0b01100000);
    assert_eq!(coord_a, expected);
}
