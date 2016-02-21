use tile::{Tile, TileRect};

// Chunk coordinates in a layer
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    tiles: Vec<Tile>,
}


impl Chunk {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Chunk {
        Chunk {
            x: x,
            y: y,
            width: w,
            height: h,
            tiles: vec![]
        }
    }

    pub fn add_tile(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.tiles.push(Tile::new(x - self.x, y - self.y, w, h));
    }

    pub fn get_tiles(&self) -> Vec<TileRect> {
        let mut result = vec![];
        for tile in self.tiles.iter() {
            result.push(tile.rect())
        }
        result
    }
}
