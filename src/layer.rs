use chunk::Chunk;
use tile::Tile;
use std::collections::HashMap;

const DEF_CHUNK_WIDTH: i32 = 100;
const DEF_CHUNK_HEIGHT: i32 = 100;

pub struct TilesLayer<T> {
    x_offset: i32,
    y_offset: i32,
    chunks: Vec<Chunk<T>>,
}

impl<T> TilesLayer<T> {
    pub fn new() -> TilesLayer<T> {
        TilesLayer {
            x_offset: 0,
            y_offset: 0,
            chunks: Vec::<Chunk<T>>::new(),
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

    pub fn add_tile(&mut self, x: i32, y: i32, w: u32, h: u32, tile_info: T) {
        let mut chunk_ind: i32 = -1;
        let rel_x = self.get_rel_x(x);
        let rel_y = self.get_rel_y(y);
        for i in 0..self.chunks.len() {
            let chunk = &self.chunks[i];
            if (rel_x > chunk.x) && (rel_x < chunk.x + chunk.width as i32) &&
               (rel_y > chunk.y) && (rel_y < chunk.y + chunk.height as i32) {
                chunk_ind = i as i32;
                break;
            }
        }
        if chunk_ind < 0 {
            self.chunks.push(Chunk::new((rel_x/DEF_CHUNK_WIDTH)*DEF_CHUNK_WIDTH,
                                        (rel_y/DEF_CHUNK_HEIGHT)*DEF_CHUNK_HEIGHT,
                                        DEF_CHUNK_WIDTH as u32,
                                        DEF_CHUNK_HEIGHT as u32));
            chunk_ind = (self.chunks.len()-1) as i32;
        }
        self.chunks[chunk_ind as usize].add_tile(rel_x, rel_y, w, h, tile_info);
    }

    pub fn get_tiles(&self, cam_x: f64, cam_y: f64, cam_w: i32, cam_h: i32) -> HashMap<(i32, i32,), &Tile<T>> {
        let rel_x = self.get_rel_x(cam_x as i32);
        let rel_y = self.get_rel_y(cam_y as i32);
        let mut result = HashMap::<(i32, i32), &Tile<T>>::new();
        for chunk in self.chunks.iter() {
            if (rel_x + cam_w > chunk.x) && (rel_x < chunk.x + chunk.width as i32) &&
               (rel_y + cam_h > chunk.y) && (rel_y < chunk.y + chunk.height as i32) {
                chunk.get_tiles(&mut result);
            }
        }
        result
    }
}
