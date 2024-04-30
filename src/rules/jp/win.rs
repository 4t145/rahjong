use crate::{draw::Draw, hand::Deck, player::Player, tile::TileId};

#[derive(Debug, Clone)]
pub enum Win {
    ChanKan { tile: TileId, from: Player },
    Ron { discard: TileId },
    Tsumo { tile: Draw },
}
#[derive(Debug, Clone)]

pub struct Tsumo {
    pub tile: Draw,
}
#[derive(Debug, Clone)]

pub struct Ron {
    pub discard: TileId,
}
#[derive(Debug, Clone)]

pub struct ChanKan {
    pub tile: TileId,
    pub from: Player,
}
