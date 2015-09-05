use tile;

pub struct Chunk {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    tiles: Vec<tile::Tile>
}


impl Chunk {
    pub fn new(x: i32, y: i32, h: i32, w: i32) -> Chunk {
        Chunk {
            x: x,
            y: y,
            width: w,
            height: h,
            tiles: vec![]
        }
    }

    // pub fn add_tile() 
}