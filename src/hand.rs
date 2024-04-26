use crate::tile::{tile_set::TileSet, Num, Suit, TileFace, TileId};
#[derive(Default)]
pub struct Hand {
    tiles: TileSet,
}

pub enum ReadyHand {
    ThirteenOrphans,
    SevenPairs,
    Normal,
}

impl Hand {
    pub fn can_thirteen_orphans(&self) -> bool {
        let mut orphans = TileSet::new();
        for tile in self.tiles.iter() {
            if tile.face().is_terminal() {
                orphans.insert(tile);
            }
        }
        orphans.len() == 13
    }

    pub fn can_seven_pairs(&self) -> bool {
        let mut pairs = 0;
        for tile in self.tiles.iter() {
            if self.tiles.count_face(tile.face()) == 2 {
                pairs += 1;
            }
        }
        pairs == 7
    }
    pub fn new() -> Self {
        Hand {
            tiles: TileSet::new(),
        }
    }

    pub fn can_pong(&self, claim: TileId) -> bool {
        self.tiles.count_face(claim.face()) >= 2
    }

    pub fn can_chow(&self, claim: TileId) -> bool {
        if let Some(suit) = claim.face().try_into_suit() {
            let num = suit.num;
            let kind = suit.kind;
            let check = |(a, b): (Num, Num)| {
                self.tiles.has_face(Suit { kind, num: a }.into())
                    && self.tiles.has_face(Suit { kind, num: b }.into())
            };
            num.next_two().is_some_and(check)
                || num.prev_two().is_some_and(check)
                || num.prev_and_next().is_some_and(check)
        } else {
            false
        }
    }

    pub fn can_kong(&self, claim: TileId) -> bool {
        self.tiles.count_face(claim.face()) >= 3
    }

    pub fn chow_options(&self, claim: TileId) -> Vec<Chow> {
        if let Some(suit) = claim.face().try_into_suit() {
            let num = suit.num;
            let kind = suit.kind;
            let mut chows = Vec::new();
            let chow = |(a, b): (Num, Num)| {
                let tiles = [Suit { kind, num: a }.into(), Suit { kind, num: b }.into()];
                Chow {
                    claim: claim.face(),
                    tiles,
                }
            };
            if let Some((a, b)) = num.next_two() {
                if self.tiles.has_face(Suit { kind, num: a }.into())
                    && self.tiles.has_face(Suit { kind, num: b }.into())
                {
                    chows.push(chow((a, b)));
                }
            }
            if let Some((a, b)) = num.prev_two() {
                if self.tiles.has_face(Suit { kind, num: a }.into())
                    && self.tiles.has_face(Suit { kind, num: b }.into())
                {
                    chows.push(chow((a, b)));
                }
            }
            if let Some((a, b)) = num.prev_and_next() {
                if self.tiles.has_face(Suit { kind, num: a }.into())
                    && self.tiles.has_face(Suit { kind, num: b }.into())
                {
                    chows.push(chow((a, b)));
                }
            }
            chows
        } else {
            Vec::new()
        }
    }
}

pub struct MeldedSet {
    pub chow: Vec<Chow>,
    pub pong: Vec<Pong>,
    pub kong: Vec<Kong>,
}

pub struct Chow {
    pub claim: TileFace,
    pub tiles: [TileFace; 2],
}

pub struct Pong {
    pub claim: TileFace,
    pub tiles: [TileFace; 2],
}

pub struct Kong {
    pub claim: TileFace,
    pub tiles: [TileFace; 3],
    pub exposed: bool,
}

pub struct Deck {
    pub hand: Hand,
    pub melded_set: MeldedSet,
}
