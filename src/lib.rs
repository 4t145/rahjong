use hand::{Deck, MeldedSet};
use tile::{TileId, Unicode};
pub mod hand;
pub mod tile;
pub mod rules;
pub trait TileSet {
    type Tile: Unicode;
    const SIZE: usize;
    fn count(tile: Self::Tile) -> usize;
}

