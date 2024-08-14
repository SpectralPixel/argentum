use std::{error::Error, fmt};

use crate::Coordinate;

#[derive(Debug, Clone)]
pub struct CoordinateOutOfBoundsError(pub Coordinate);

impl fmt::Display for CoordinateOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Coordinate {} is out of bounds! (integer overflow)",
            self.0
        )
    }
}

impl Error for CoordinateOutOfBoundsError {}
