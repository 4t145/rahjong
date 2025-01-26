use std::collections::HashSet;

use crate::tile::TileFace;

pub trait TileSet {
    fn faces(&self) -> HashSet<TileFace>;
}

