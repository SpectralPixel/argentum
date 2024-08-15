use thiserror::Error;

use crate::Coordinate;

#[derive(Debug, Clone, Error)]
#[error("Coordinate {0} is experiencing integer overflow.")]
pub struct CoordinateOverflowError(pub Coordinate);
