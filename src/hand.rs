use crate::tile::TileId;

pub struct Hand {
    tiles: Vec<TileId>,
}

impl Hand {
    pub fn sort(&mut self) {
        self.tiles.sort();
    }
    pub fn check_chow(&self, tile: TileId) {
        // find prev and next
        self.tiles.binary_search(x)
    }
}

pub struct MeldedSet {
    pub chow: Vec<Chow>,
    pub pong: Vec<Pong>,
    pub kong: Vec<Kong>,
}

pub struct Chow {
    pub claim: TileId,
    pub tiles: [TileId; 3],
}

pub struct Pong {
    pub claim: TileId,
    pub tiles: [TileId; 3],
}

pub struct Kong {
    pub claim: TileId,
    pub tiles: [TileId; 4],
    pub exposed: bool,
}

pub struct Deck {
    pub hand: Hand,
    pub melded_set: MeldedSet,
}
