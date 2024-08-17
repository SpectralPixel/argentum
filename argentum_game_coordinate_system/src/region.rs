use std::num::NonZeroU16;

use crate::{Coordinate, CoordinateType};

type SizeType = NonZeroU16;

/// Cube-shaped iterator of `Coordinate`s
///
/// The iterator returns all `Coordinate`s from `position` to `position + size - 1`
///
/// # Examples
///
/// ```
/// # fn main() {
/// #     test().unwrap();
/// # }
/// # fn test() -> Option<()> {
/// use std::num::NonZeroU16;
/// use argentum_game_coordinate_system::{Coordinate, Region};
///
/// let mut positions: Vec<Coordinate> = Vec::new();
/// positions.push(Coordinate::new(7, 7, 7));
/// positions.push(Coordinate::new(8, 7, 7));
/// positions.push(Coordinate::new(7, 8, 7));
/// positions.push(Coordinate::new(8, 8, 7));
/// positions.push(Coordinate::new(7, 7, 8));
/// positions.push(Coordinate::new(8, 7, 8));
/// positions.push(Coordinate::new(7, 8, 8));
/// positions.push(Coordinate::new(8, 8, 8));
///
/// let mut i = 0;
/// let region = Region::new(Coordinate::splat(7), NonZeroU16::new(2)?);
/// for pos in region {
///     println!("{i}");
///     assert_eq!(pos, positions[i]);
///     i += 1;
/// }
/// # Some(())
/// # }
/// ```
///
/// # Errors
///
/// Setting a `size` that is less than or equal to `0` will cause the program to crash.
#[derive(PartialEq, Debug)]
pub struct Region {
    position: Coordinate,
    size: SizeType,
    offset: Coordinate,
    first_iteration: bool,
}

impl Region {
    /// Creates a new Region
    ///
    /// - `position` corresponds to the starting position of the iterator.
    /// - `size` detemines the range of the iterator. The range is exclusive. Must be larger than `0`.
    pub fn new(position: Coordinate, size: SizeType) -> Self {
        Self {
            position,
            size: SizeType::from(size),
            offset: Coordinate::new(0, 0, 0),
            first_iteration: true,
        }
    }

    /// Returns the `Region`'s size.
    pub fn size(&self) -> SizeType {
        self.size
    }
}

impl Iterator for Region {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iteration {
            self.first_iteration = false;
            return Some(self.position.to_owned());
        }

        let size = CoordinateType::try_from(u16::from(self.size())).unwrap();

        fn increment(n: &mut CoordinateType, size: CoordinateType) -> bool {
            *n = (*n + 1) % size;
            *n == 0
        }

        let wrapped_x = increment(&mut self.offset.x, size);
        let mut wrapped_y = false;
        let mut wrapped_z = false;

        if wrapped_x {
            wrapped_y = increment(&mut self.offset.y, size);
        }
        if wrapped_y {
            wrapped_z = increment(&mut self.offset.z, size);
        }
        match wrapped_z {
            false => Some(self.position.to_owned() + self.offset.to_owned()),
            true => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new(pos: Coordinate, size: NonZeroU16) -> bool {
            let result =  Region::new(pos.clone(), size);
            let expected = Region {
                position: pos,
                size,
                offset: Coordinate::new(0, 0, 0),
                first_iteration: true,
            };
            result == expected
        }
    }

    #[test]
    #[should_panic]
    fn zero_size() {
        let region = Region::new(Coordinate::splat(0), NonZeroU16::new(0).unwrap());
        
        let mut i = 0;
        for pos in region {
            println!("{pos}");
            i += 1;
        }
        assert!(i < 0, "Never even iterated!");
    }
}
