use tile::TileType;
use chunk::Chunk;

const DEF_CHUNK_WIDTH: i32 = 100;
const DEF_CHUNK_HEIGHT: i32 = 100;

pub struct TilesLayer {
    x_offset: i32,
    y_offset: i32,
    chunks: Vec<Chunk>,
}

impl TilesLayer {
    pub fn new() -> TilesLayer {
        TilesLayer {
            x_offset: 0,
            y_offset: 0,
            chunks: Vec::<Chunk>::new(),
        }
    }

    #[inline]
    fn get_rel_x(&self, x: i32) -> i32 {
        x - self.x_offset
    }

    #[inline]
    fn get_rel_y(&self, y: i32) -> i32 {
        y - self.y_offset
    }

    pub fn add_tile(&mut self, x: i32, y: i32, w: u32, h: u32, tile_type: TileType, texture_ind: i32) {
        let mut chunk_exists = false;
        let rel_x = self.get_rel_x(x);
        let rel_y = self.get_rel_y(y);
        for chunk in self.chunks.iter_mut() {
            if (rel_x > chunk.x) && (rel_x < chunk.x + chunk.width as i32) &&
               (rel_y > chunk.y) && (rel_y < chunk.y + chunk.height as i32) {
                chunk_exists = true;
                chunk.add_tile(rel_x, rel_y, w, h, tile_type, texture_ind);
                break;
            }
        }
        if !chunk_exists {
            self.chunks.push(Chunk::new((rel_x/DEF_CHUNK_WIDTH)*DEF_CHUNK_WIDTH,
                                        (rel_y/DEF_CHUNK_HEIGHT)*DEF_CHUNK_HEIGHT,
                                        DEF_CHUNK_WIDTH as u32,
                                        DEF_CHUNK_HEIGHT as u32));
            self.chunks.last_mut().unwrap().add_tile(rel_x, rel_y, w, h, tile_type, texture_ind);
        }
    }

    pub fn get_tiles(&self, cam_x: f64, cam_y: f64, cam_w: i32, cam_h: i32) -> Vec<&::tile::Tile> {
        let rel_x = self.get_rel_x(cam_x as i32);
        let rel_y = self.get_rel_y(cam_y as i32);
        let mut result = vec![];
        for chunk in self.chunks.iter() {
            if (rel_x + cam_w > chunk.x) && (rel_x < chunk.x + chunk.width as i32) &&
               (rel_y + cam_h > chunk.y) && (rel_y < chunk.y + chunk.height as i32) {
                chunk.get_tiles(&mut result);
            }
        }
        result
    }
}
