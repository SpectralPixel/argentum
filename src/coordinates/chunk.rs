// i16: From −32,768 to 32,767
// This should be enough chunks, right?
type ChunkMaxSize = i16;

pub struct ChunkCoord {
    pub x: ChunkMaxSize,
    pub y: ChunkMaxSize,
    pub z: ChunkMaxSize,
}
