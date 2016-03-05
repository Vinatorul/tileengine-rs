// Tile coodinates in a chunk
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile<T> {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub tile_info: T,
}

impl<T> Tile<T> {
    pub fn new(x: i32, y: i32, w: u32, h: u32, tile_info: T) -> Tile<T> {
        Tile {
            x: x,
            y: y,
            width: w,
            height: h,
            tile_info: tile_info,
        }
    }
}
