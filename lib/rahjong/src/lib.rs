use tile::TileId;
pub mod hand;
pub mod rules;
pub mod tile;
pub mod wall;
pub mod player;
pub mod discard;
pub mod draw;
pub mod game;

pub trait Tiles {
    fn new_wall(&self) -> Vec<TileId>;
    fn size(&self) -> usize;
}
