use crate::{hand::Deck, tile::TileId};

pub mod jp;


pub trait Rule {
    fn check_win(deck: &Deck, tile: TileId) -> bool;
}