#[test]
fn get_rect() {
	use tile;
    let tile = tile::Tile::new(1, 2, 3, 4);
    assert_eq!(tile.rect(), [1, 2, 3, 4]);
}