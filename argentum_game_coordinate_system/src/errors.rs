use thiserror::Error;

use crate::Coordinate;

#[derive(Debug, Clone, Error)]
#[error("{0} is experiencing integer overflow.")]
pub struct CoordinateOverflowError(pub Coordinate);
