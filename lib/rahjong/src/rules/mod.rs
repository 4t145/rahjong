use crate::{hand::Deck, tile::TileId};

pub mod jp;
pub mod sc;


pub trait Rule {
    fn check_win(deck: &Deck, tile: TileId) -> bool;
}
