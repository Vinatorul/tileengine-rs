use layer::TilesLayer;
use std::collections::HashMap;
use tile::{Tile, TileType};

#[derive(Default)]
pub struct TileEngine {
    layers: HashMap<i32, TilesLayer>,
}

impl TileEngine {
    pub fn add_tile(&mut self, x: i32, y: i32, w: u32, h: u32, layer_ind: i32, tile_type: TileType, texture_ind: i32) {
        if let Some(l) = self.layers.get_mut(&layer_ind) {
            l.add_tile(x, y, w, h, tile_type, texture_ind);
            return;
        }
        self.layers.insert(layer_ind, TilesLayer::new());
        self.add_tile(x, y, w, h, layer_ind, tile_type, texture_ind);
    }

    pub fn get_tiles(&self, cam_x: f64, cam_y: f64, cam_w: i32, cam_h: i32, layer_ind: i32) -> Vec<&Tile> {
        let layer = self.layers.get(&layer_ind).unwrap_or_else(|| {panic!("Layer {} not found", layer_ind)});
        layer.get_tiles(cam_x, cam_y, cam_w, cam_h)
    }
}
