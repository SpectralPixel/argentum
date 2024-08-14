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
        fn new_region(position: Coordinate, size: CoordinateType) -> bool {
            let result = Region::new(position.clone(), size);
            let expected = Region { position, size, offset: Coordinate::new(0,0,0), };
            result == expected
        }
    }
}
