use crate::{Coordinate, CoordinateType};

#[derive(PartialEq, Debug)]
pub struct Region {
    position: Coordinate,
    size: CoordinateType,
    offset: Coordinate,
    first_iteration: bool,
}

impl Region {
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
            return Some(self.position.to_owned())
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

    #[test]
    fn iterate() {
        let region = Region::new(Coordinate::new(0, 0, 0), 10);
        region
            .into_iter()
            .filter(|pos| pos.x == 0)
            .for_each(|pos| assert!(pos.x == 0));
    }
}
