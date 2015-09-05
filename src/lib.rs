#![crate_name = "tile_engine"]
#![crate_type = "lib"]

extern crate xml;

mod tile;
mod core;
mod reader;
mod chunk;
mod layer;

#[cfg(test)]
mod tests; 