use crate::tile::TileId;
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Draw {
    tile: TileId
}