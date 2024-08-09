// u8: From 0 to 255
// Might need to increase this number if the chunk size grows beyond 255.
type LocalMaxSize = u8;

#[derive(PartialEq, Debug)]
pub struct LocalCoord {
    pub x: LocalMaxSize,
    pub y: LocalMaxSize,
    pub z: LocalMaxSize,
}

impl LocalCoord {
    pub fn new(x: LocalMaxSize, y: LocalMaxSize, z: LocalMaxSize) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: LocalMaxSize, y: LocalMaxSize, z: LocalMaxSize) -> bool {
            let result = LocalCoord::new(x, y, z);
            let expected = LocalCoord { x, y, z };
            result == expected
        }
    }
}
