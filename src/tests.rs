#[test]
fn get_rect() {
    use tile;
    let tile = tile::Tile::new(1, 2, 3, 4);
    assert_eq!(tile.rect(0, 0), [1, 2, 3, 4]);
}

#[test]
fn get_rect_offset() {
    use tile;
    let tile = tile::Tile::new(1, 2, 3, 4);
    assert_eq!(tile.rect(30, 100), [31, 102, 3, 4]);
}

#[test]
fn get_rect_neg() {
    use tile;
    let tile = tile::Tile::new(1, 2, 3, 4);
    assert_eq!(tile.rect(-100, -299), [-99, -297, 3, 4]);
}
