#[derive(PartialEq, Debug)]
struct Cube {
    index: u8,
}

impl Cube {
    pub fn new(index: u8) -> Self {
        Cube { index }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cube() {
        let index = 2;
        let result = Cube::new(index);
        let expected = Cube { index };
        assert_eq!(result, expected);
    }
}
