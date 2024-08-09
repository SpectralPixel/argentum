// u8: From 0 to 255
// Might need to increase this number if the chunk size grows beyond 255.
type LocalMaxSize = u8;

pub struct LocalCoord {
    pub x: LocalMaxSize,
    pub y: LocalMaxSize,
    pub z: LocalMaxSize,
}
