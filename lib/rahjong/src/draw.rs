use crate::tile::TileId;
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Draw {
    tile: TileId
}

impl Draw {
    pub fn new(tile: TileId) -> Self {
        Draw { tile }
    }
    pub fn tile(&self) -> TileId {
        self.tile
    }
    pub fn into_tile(self) -> TileId {
        self.tile
    }
}