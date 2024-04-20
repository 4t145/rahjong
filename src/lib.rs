use tile::Unicode;
pub mod hand;
pub mod rules;
pub mod tile;
pub trait TileSet {
    type Tile: Unicode;
    const SIZE: usize;
    fn count(tile: Self::Tile) -> usize;
}
