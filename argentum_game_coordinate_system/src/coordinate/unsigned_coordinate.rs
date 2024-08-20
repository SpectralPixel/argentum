use num::{Num, Unsigned};

#[cfg(test)]
mod tests;

struct UnsignedCoordinate<T>
where
    T: Num + Unsigned
{
    pub x: T,
    pub y: T,
    pub z: T,
}