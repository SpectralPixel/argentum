use thiserror::Error;

use crate::Coordinate;

#[derive(Debug, Clone, Error)]
#[error("Coordinate {0} is out of bounds! (integer overflow)")]
pub struct CoordinateOutOfBoundsError(pub Coordinate);
