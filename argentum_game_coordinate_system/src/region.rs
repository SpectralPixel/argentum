use crate::CoordinateType;

#[derive(PartialEq, Debug)]
pub struct Region {
    size: CoordinateType,
}

impl Region {
    pub fn new(size: CoordinateType) -> Self {
        Self { size }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_region(size: CoordinateType) -> bool {
            let result = Region::new(size);
            let expected = Region { size };
            result == expected
        }
    }
}
