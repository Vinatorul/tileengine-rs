use chunk::Chunk;

const DEF_CHUNK_WIDTH: i32 = 100;
const DEF_CHUNK_HEIGHT: i32 = 100;

pub struct TilesLayer {
    x_offset: f64,
    y_offset: f64,
    chunks: Vec<Chunk>,
}

impl TilesLayer {
    pub fn new() -> TilesLayer {
        TilesLayer {
            x_offset: 0.,
            y_offset: 0.,
            chunks: Vec::<Chunk>::new(),
        }
    }

    #[inline]
    fn get_rel_x(&self, x: f64) -> i32 {
        (x - self.x_offset) as i32
    }

    #[inline]
    fn get_rel_y(&self, y: f64) -> i32 {
        (y - self.y_offset) as i32
    }

    pub fn add_tile(&mut self, x: f64, y: f64, w: i32, h: i32) {
        let mut chunk_exists = false;
        let rel_x = self.get_rel_x(x);
        let rel_y = self.get_rel_y(y); 
        for chunk in self.chunks.iter_mut() {
            if (rel_x > chunk.x) && (rel_x < chunk.x + chunk.width) &&
               (rel_y > chunk.y) && (rel_y < chunk.y + chunk.height) {
                chunk_exists = true;
                chunk.add_tile(rel_x, rel_y, w, h);
                break;
            }
        }
        if !chunk_exists {
            self.chunks.push(Chunk::new((rel_x/DEF_CHUNK_WIDTH)*DEF_CHUNK_WIDTH, 
                                        (rel_y/DEF_CHUNK_HEIGHT)*DEF_CHUNK_HEIGHT, 
                                        DEF_CHUNK_WIDTH, 
                                        DEF_CHUNK_HEIGHT));
            self.chunks.last_mut().unwrap().add_tile(rel_x, rel_y, w, h);
        }
    }

    pub fn get_tiles(&self, cam_x: f64, cam_y: f64, cam_w: i32, cam_h: i32) -> Vec<::tile::TileRect> {
        let rel_x = self.get_rel_x(cam_x);
        let rel_y = self.get_rel_y(cam_y);
        let mut result = vec![];
        for chunk in self.chunks.iter() {
            if (rel_x + cam_w > chunk.x) && (rel_x < chunk.x + chunk.width) &&
               (rel_y + cam_h > chunk.y) && (rel_y < chunk.y + chunk.height) {
                result.append(chunk.get_tiles().as_mut());
            }
        }
        result
    }
}