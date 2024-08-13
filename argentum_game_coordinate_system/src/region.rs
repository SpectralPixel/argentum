use crate::{Coordinate, CoordinateType};

#[derive(PartialEq, Debug)]
pub struct Region {
    position: Coordinate,
    size: CoordinateType,
    offset: Coordinate,
}

impl Region {
    pub fn new(position: Coordinate, size: CoordinateType) -> Self {
        Self {
            position,
            size,
            offset: Coordinate::new(0, 0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_region(size: CoordinateType) -> bool {
            let coordinate = Coordinate::new(0,0,0);
            let result = Region::new(coordinate.clone(), size);
            let expected = Region { position: coordinate.clone(), size, offset: coordinate.clone() };
            result == expected
        }
    }
}
