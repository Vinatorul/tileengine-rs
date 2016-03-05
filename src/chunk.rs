use tile::{Tile, TileType};

// Chunk coordinates in a layer
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}


impl Chunk {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Chunk {
        Chunk {
            x: x,
            y: y,
            width: w,
            height: h,
            tiles: vec![]
        }
    }

    pub fn add_tile(&mut self, x: i32, y: i32, w: u32, h: u32, tile_type: TileType, texture_ind: i32) {
        self.tiles.push(Tile::new(x, y, w, h, tile_type, texture_ind));
    }

    pub fn get_tiles<'a>(&'a self, tiles: &mut Vec<&'a Tile>) {
        for tile in self.tiles.iter() {
            tiles.push(&tile);
        }
    }
}
