use crate::{player::PlayerId, tile::TileId, wind::Wind};
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Discard {
    pub tile: TileId,
    pub player: PlayerId,
}
