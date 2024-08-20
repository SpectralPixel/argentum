use num::Num;

struct BaseCoordinate<T>
where
    T: Num
{
    pub x: T,
    pub y: T,
    pub z: T,
}