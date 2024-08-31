use quickcheck::quickcheck;

use super::*;

quickcheck! {
    fn new(x: isize, y: isize, z: isize) -> bool {
        let result = SignedCoordinate::new(x, y, z);
        let expected = SignedCoordinate { x, y, z };
        result == expected
    }
}

quickcheck! {
    fn arbitrary(coord: SignedCoordinate<isize>) -> bool {
        let x_fine = coord.x >= SignedCoordinate::MIN.x && coord.x <= SignedCoordinate::MAX.x;
        let y_fine = coord.y >= SignedCoordinate::MIN.y && coord.y <= SignedCoordinate::MAX.y;
        let z_fine = coord.z >= SignedCoordinate::MIN.z && coord.z <= SignedCoordinate::MAX.z;
        x_fine && y_fine && z_fine
    }
}

#[test]
fn min_pos() {
    let expected = SignedCoordinate {
        x: isize::MIN,
        y: isize::MIN,
        z: isize::MIN,
    };
    assert_eq!(expected, SignedCoordinate::MIN);
    assert_eq!(expected.x, isize::MIN);
    assert_eq!(expected.y, isize::MIN);
    assert_eq!(expected.z, isize::MIN);
}

#[test]
fn max_pos() {
    let expected = SignedCoordinate {
        x: isize::MAX,
        y: isize::MAX,
        z: isize::MAX,
    };
    assert_eq!(expected, SignedCoordinate::MAX);
    assert_eq!(expected.x, isize::MAX);
    assert_eq!(expected.y, isize::MAX);
    assert_eq!(expected.z, isize::MAX);
}

#[test]
fn display() {
    let pos = SignedCoordinate { x: 1, y: 2, z: 3 };

    assert_eq!(pos.to_string(), "(SignedCoordinate: 1, 2, 3)")
}

quickcheck! {
    fn splat(n: isize) -> bool {
        let coord = SignedCoordinate::splat(n);
        coord.x == coord.y && coord.y == coord.z
    }
}

#[test]
fn add() {
    let result = SignedCoordinate::new(1, 2, 3) + SignedCoordinate::new(5, 0, 10);
    let expected = SignedCoordinate::new(6, 2, 13);
    assert_eq!(result, expected);
}

#[test]
fn add_single() {
    let result = SignedCoordinate::new(1, 2, 3) + 5;
    let expected = SignedCoordinate::new(6, 7, 8);
    assert_eq!(result, expected);
}

#[test]
fn add_assign() {
    let mut result = SignedCoordinate::new(10, 15, 30);
    result += SignedCoordinate::new(-5, 10, 23);
    let expected = SignedCoordinate::new(5, 25, 53);
    assert_eq!(result, expected);
}

#[test]
fn add_assign_single() {
    let mut result = SignedCoordinate::new(-15, 15, 30);
    result += 17;
    let expected = SignedCoordinate::new(2, 32, 47);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn add_overflow() {
    let _ = SignedCoordinate::MAX + SignedCoordinate::splat(1);
}

#[test]
#[should_panic]
fn add_overflow_single() {
    let _ = SignedCoordinate::MAX + 1;
}

#[test]
fn sub() {
    let result = SignedCoordinate::new(1, 2, 3) - SignedCoordinate::new(5, 0, 10);
    let expected = SignedCoordinate::new(-4, 2, -7);
    assert_eq!(result, expected);
}

#[test]
fn sub_single() {
    let result = SignedCoordinate::new(1, 2, 3) - 2;
    let expected = SignedCoordinate::new(-1, 0, 1);
    assert_eq!(result, expected);
}

#[test]
fn sub_assign() {
    let mut result = SignedCoordinate::new(10, 15, 30);
    result -= SignedCoordinate::new(-5, 10, 23);
    let expected = SignedCoordinate::new(15, 5, 7);
    assert_eq!(result, expected);
}

#[test]
fn sub_assign_single() {
    let mut result = SignedCoordinate::new(10, 16, 30);
    result -= 7;
    let expected = SignedCoordinate::new(3, 9, 23);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn sub_overflow() {
    let _ = SignedCoordinate::MIN - SignedCoordinate::splat(1);
}

#[test]
#[should_panic]
fn sub_overflow_single() {
    let _ = SignedCoordinate::MIN - 1;
}

#[test]
fn mul() {
    let result = SignedCoordinate::new(5, 2, 4) * SignedCoordinate::new(1, 0, 2);
    let expected = SignedCoordinate::new(5, 0, 8);
    assert_eq!(result, expected);
}

#[test]
fn mul_single() {
    let result = SignedCoordinate::new(5, 2, 4) * 1;
    let expected = SignedCoordinate::new(5, 2, 4);
    assert_eq!(result, expected);
}

#[test]
fn mul_assign() {
    let mut result = SignedCoordinate::new(3, 360, 2);
    result *= SignedCoordinate::new(-5, 2, 7);
    let expected = SignedCoordinate::new(-15, 720, 14);
    assert_eq!(result, expected);
}

#[test]
fn mul_assign_single() {
    let mut result = SignedCoordinate::new(3, 360, -2);
    result *= -2;
    let expected = SignedCoordinate::new(-6, -720, 4);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn mul_overflow() {
    let _ = SignedCoordinate::MAX * SignedCoordinate::splat(2);
}

#[test]
#[should_panic]
fn mul_overflow_single() {
    let _ = SignedCoordinate::MAX * 2;
}

#[test]
fn div() {
    let result = SignedCoordinate::new(5, 2, 4) / SignedCoordinate::new(1, 2, 2);
    let expected = SignedCoordinate::new(5, 1, 2);
    assert_eq!(result, expected);
}

#[test]
fn div_single() {
    let result = SignedCoordinate::new(8, 2, 4) / 2;
    let expected = SignedCoordinate::new(4, 1, 2);
    assert_eq!(result, expected);
}

#[test]
fn div_assign() {
    let mut result = SignedCoordinate::new(5, 360, 24);
    result /= SignedCoordinate::new(-5, 2, 4);
    let expected = SignedCoordinate::new(-1, 180, 6);
    assert_eq!(result, expected);
}

#[test]
fn div_assign_single() {
    let mut result = SignedCoordinate::new(5, -360, 24);
    result /= -2;
    let expected = SignedCoordinate::new(-2, 180, -12);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn div_by_zero() {
    let _ = SignedCoordinate::MAX / SignedCoordinate::splat(0);
}

#[test]
#[should_panic]
fn div_by_zero_single() {
    let _ = SignedCoordinate::MAX / 0;
}

#[test]
fn rem() {
    let result = SignedCoordinate::new(7, -12, 4) % SignedCoordinate::new(5, 7, 2);
    let expected = SignedCoordinate::new(2, -5, 0);
    assert_eq!(result, expected);
}

#[test]
fn rem_single() {
    let result = SignedCoordinate::new(7, -2, 4) % 5;
    let expected = SignedCoordinate::new(2, -2, 4);
    assert_eq!(result, expected);
}

#[test]
fn rem_assign() {
    let mut result = SignedCoordinate::new(7, -11, 38);
    result %= SignedCoordinate::splat(6);
    let expected = SignedCoordinate::new(1, -5, 2);
    assert_eq!(result, expected);
}

#[test]
fn rem_assign_single() {
    let mut result = SignedCoordinate::new(7, -11, 38);
    result %= 6;
    let expected = SignedCoordinate::new(1, -5, 2);
    assert_eq!(result, expected);
}

#[test]
fn neg() {
    let result = -SignedCoordinate::splat(7);
    let expected = SignedCoordinate::splat(-7);
    assert_eq!(result, expected);
}

#[test]
fn not() {
    let result = !SignedCoordinate::<isize>::MAX;
    let expected = SignedCoordinate::MIN;
    assert_eq!(result, expected);
}

#[test]
fn bit_and() {
    let coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = SignedCoordinate::splat(0b11001100);
    let result = coord_a & coord_b;
    let expected = SignedCoordinate::new(0b10001000, 0b00001100, 0b10001100);
    assert_eq!(result, expected);
}

#[test]
fn bit_and_single() {
    let coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = 0b11001100;
    let result = coord_a & coord_b;
    let expected = SignedCoordinate::new(0b10001000, 0b00001100, 0b10001100);
    assert_eq!(result, expected);
}

#[test]
fn bit_and_assign() {
    let mut coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = SignedCoordinate::splat(0b11001100);
    coord_a &= coord_b;
    let expected = SignedCoordinate::new(0b10001000, 0b00001100, 0b10001100);
    assert_eq!(coord_a, expected);
}

#[test]
fn bit_and_assign_single() {
    let mut coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = 0b11001100;
    coord_a &= coord_b;
    let expected = SignedCoordinate::new(0b10001000, 0b00001100, 0b10001100);
    assert_eq!(coord_a, expected);
}

#[test]
fn bit_or() {
    let coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = SignedCoordinate::splat(0b11001100);
    let result = coord_a | coord_b;
    let expected = SignedCoordinate::new(0b11101110, 0b11001111, 0b11101100);
    assert_eq!(result, expected);
}

#[test]
fn bit_or_single() {
    let coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = 0b11001100;
    let result = coord_a | coord_b;
    let expected = SignedCoordinate::new(0b11101110, 0b11001111, 0b11101100);
    assert_eq!(result, expected);
}

#[test]
fn bit_or_assign() {
    let mut coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = SignedCoordinate::splat(0b11001100);
    coord_a |= coord_b;
    let expected = SignedCoordinate::new(0b11101110, 0b11001111, 0b11101100);
    assert_eq!(coord_a, expected);
}

#[test]
fn bit_or_assign_single() {
    let mut coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = 0b11001100;
    coord_a |= coord_b;
    let expected = SignedCoordinate::new(0b11101110, 0b11001111, 0b11101100);
    assert_eq!(coord_a, expected);
}

#[test]
fn bit_xor() {
    let coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = SignedCoordinate::splat(0b11001100);
    let result = coord_a ^ coord_b;
    let expected = SignedCoordinate::new(0b01100110, 0b11000011, 0b01100000);
    assert_eq!(result, expected);
}

#[test]
fn bit_xor_single() {
    let coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = 0b11001100;
    let result = coord_a ^ coord_b;
    let expected = SignedCoordinate::new(0b01100110, 0b11000011, 0b01100000);
    assert_eq!(result, expected);
}

#[test]
fn bit_xor_assign() {
    let mut coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = SignedCoordinate::splat(0b11001100);
    coord_a ^= coord_b;
    let expected = SignedCoordinate::new(0b01100110, 0b11000011, 0b01100000);
    assert_eq!(coord_a, expected);
}

#[test]
fn bit_xor_assign_single() {
    let mut coord_a = SignedCoordinate::new(0b10101010, 0b00001111, 0b10101100);
    let coord_b = 0b11001100;
    coord_a ^= coord_b;
    let expected = SignedCoordinate::new(0b01100110, 0b11000011, 0b01100000);
    assert_eq!(coord_a, expected);
}
