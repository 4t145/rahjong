use crate::{hand::Deck, tile::TileId};

pub enum WinKind {
    Ron,
    Tsumo,
}
pub struct Win<'d> {
    deck: &'d Deck,
    tile: TileId,
    kind: WinKind,
}

pub trait WinCheck {
    fn check(deck: &Deck, tile: TileId, kind: WinKind) -> bool;
}


pub struct Plain;
