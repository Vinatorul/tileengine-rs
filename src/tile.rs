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
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    tile_type: TileType,
    texture_ind: i32,
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
