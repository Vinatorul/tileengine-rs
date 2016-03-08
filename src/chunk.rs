use tile::Tile;
use std::collections::HashMap;

// Chunk coordinates in a layer
pub struct Chunk<T> {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile<T>>,
}


impl<T> Chunk<T> {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Chunk<T> {
        Chunk {
            x: x,
            y: y,
            width: w,
            height: h,
            tiles: vec![]
        }
    }

    pub fn add_tile(&mut self, x: i32, y: i32, w: u32, h: u32, tile_info: T) {
        self.tiles.push(Tile::new(x, y, w, h, tile_info));
    }

    pub fn get_tiles<'a>(&'a self, tiles: &mut HashMap<(i32, i32), &'a Tile<T>>) {
        for tile in self.tiles.iter() {
            tiles.insert((tile.x, tile.y) ,&tile);
        }
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Option<&Tile<T>> {
        for tile in self.tiles.iter() {
            if (tile.x == x) && (tile.y == y) {
                return Some(&tile);
            }
        }
        None
    }
}
