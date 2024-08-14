use crate::{Coordinate, CoordinateType};

/// Cube-shaped iterator of `Coordinate`s
///
/// The iterator returns all `Coordinate`s from `position` to `position + size - 1`
///
/// # Examples
///
/// ```
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
/// let region = Region::new(Coordinate::splat(7), 2);
/// for pos in region {
///     println!("{i}");
///     assert_eq!(pos, positions[i]);
///     i += 1;
/// }
/// ```
#[derive(PartialEq, Debug)]
pub struct Region {
    position: Coordinate,
    size: CoordinateType,
    offset: Coordinate,
    first_iteration: bool,
}

impl Region {
    /// Creates a new Region
    ///
    /// - `position` corresponds to the starting position of the iterator.
    /// - `size` detemines the range of the iterator. The range is exclusive.
    pub fn new(position: Coordinate, size: CoordinateType) -> Self {
        Self {
            position,
            size,
            offset: Coordinate::new(0, 0, 0),
            first_iteration: true,
        }
    }
}

impl Iterator for Region {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iteration {
            self.first_iteration = false;
            return Some(self.position.to_owned());
        }

        fn increment(n: &mut CoordinateType, size: CoordinateType) -> bool {
            *n = (*n + 1) % size;
            *n == 0
        }

        let wrapped_x = increment(&mut self.offset.x, self.size);
        let mut wrapped_y = false;
        let mut wrapped_z = false;

        if wrapped_x {
            wrapped_y = increment(&mut self.offset.y, self.size);
        }
        if wrapped_y {
            wrapped_z = increment(&mut self.offset.z, self.size);
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
        fn new_region(position: Coordinate, size: CoordinateType) -> bool {
            let result = Region::new(position.clone(), size);
            let expected = Region { position, size, offset: Coordinate::new(0, 0, 0), first_iteration: true };
            result == expected
        }
    }
}
