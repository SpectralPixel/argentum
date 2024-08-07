#[derive(PartialEq, Debug, Default, Clone)]
pub struct Cube {
    index: u8,
}

impl Cube {
    pub fn new(index: u8) -> Self {
        Cube { index }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_cube(index: u8) -> bool {
            let result = Cube::new(index);
            let expected = Cube { index };
            result == expected
        }
    }
}
