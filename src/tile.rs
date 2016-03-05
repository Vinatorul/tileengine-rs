#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    None,
    Wall,
    Floor,
    Door(bool),
}

// Tile coodinates in a chunk
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub tile_type: TileType,
    pub texture_ind: i32,
}

impl Tile {
    pub fn new(x: i32, y: i32, w: u32, h: u32, tile_type: TileType, texture_ind: i32) -> Tile {
        Tile {
            x: x,
            y: y,
            width: w,
            height: h,
            tile_type: tile_type,
            texture_ind: texture_ind,
        }
    }
}
