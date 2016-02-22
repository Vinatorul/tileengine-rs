pub type TileRect = [i32; 4];

// Tile coodinates in a chunk
pub struct Tile {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Tile {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Tile {
        Tile {
            x: x,
            y: y,
            width: w,
            height: h
        }
    }

    pub fn rect(&self, x_offset: i32, y_offset: i32) -> TileRect {
        [self.x + x_offset, self.y + y_offset, self.width, self.height]
    }
}
