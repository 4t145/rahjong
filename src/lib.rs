use tile::TileId;
pub mod hand;
pub mod rules;
pub mod tile;
pub mod wall;
pub trait Tiles {
    fn new_wall(&self) -> Vec<TileId>;
    fn size(&self) -> usize;
}
