// i32: From −2,147,483,648 to 2,147,483,647
// I don't believe a larger size is necessary, as the RAM usage per instance
// would double. Heck, even this is already overkill.
type GlobalMaxSize = i32;

pub struct GlobalCoord {
    pub x: GlobalMaxSize,
    pub y: GlobalMaxSize,
    pub z: GlobalMaxSize,
}
