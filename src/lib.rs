#![crate_name = "tile_engine"]
#![crate_type = "lib"]

mod tile;
mod core;
mod chunk;
mod layer;

pub use core::TileEngine;
pub use tile::{Tile, TileType};

#[cfg(test)]
mod tests;
