use layer;

struct TileEngine<'m> {
    map: &'m str,
    layers: Vec<layer::TilesLayer>
}