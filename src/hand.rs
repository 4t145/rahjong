use crate::{
    discard::Discard,
    tile::{
        tile_set::{TileIndexSet, TileSet},
        Num, Suit, TileFace, TileId,
    },
};
#[derive(Debug, Default, Clone)]
pub struct Hand {
    tiles: TileSet,
}

pub enum ReadyHand {
    ThirteenOrphans,
    SevenPairs,
    Normal,
}

impl Hand {
    pub fn contains(&self, tile: TileId) -> bool {
        self.tiles.has(tile)
    }
    pub fn remove(&mut self, tile: TileId) {
        self.tiles.remove(tile);
    }
    pub fn can_thirteen_orphans(&self) -> bool {
        let mut stat = [0; 13];
        let index = |face: TileFace| -> Option<usize> {
            match face {
                crate::tile::RED => Some(0),
                crate::tile::GREEN => Some(1),
                crate::tile::WHITE => Some(2),
                crate::tile::EAST => Some(3),
                crate::tile::SOUTH => Some(4),
                crate::tile::WEST => Some(5),
                crate::tile::NORTH => Some(6),
                crate::tile::B1 => Some(7),
                crate::tile::B9 => Some(8),
                crate::tile::C1 => Some(9),
                crate::tile::C9 => Some(10),
                crate::tile::D1 => Some(11),
                crate::tile::D9 => Some(12),
                _ => None,
            }
        };

        for tile in self.tiles.iter() {
            let Some(idx) = index(tile.face()) else {
                return false;
            };
            stat[idx] += 1;
            if stat[idx] > 2 {
                return false;
            }
        }
        let mut zero_count = 0;
        for count in stat.iter() {
            if *count == 0 {
                zero_count += 1;
                if zero_count > 1 {
                    return false;
                }
            }
        }
        zero_count <= 1
    }

    pub fn can_seven_pairs(&self) -> bool {
        let mut pairs = TileSet::new();

        for tile in self.tiles.iter() {
            if self.tiles.count_face(tile.face()) == 2 {
                pairs.insert(tile)
            }
        }
        pairs.len() == 12
    }
    pub fn new(iter: impl IntoIterator<Item = TileId>) -> Self {
        Hand {
            tiles: iter.into_iter().collect(),
        }
    }

    pub fn can_peng(&self, claim: TileId) -> bool {
        self.tiles.count_face(claim.face()) >= 2
    }

    pub fn can_chi(&self, claim: TileId) -> bool {
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

    pub fn can_gang(&self, claim: TileId) -> bool {
        self.tiles.count_face(claim.face()) >= 3
    }

    pub fn chi_options(&self, claim: TileId) -> Vec<ChiOptions> {
        if let Some(suit) = claim.face().try_into_suit() {
            let num = suit.num;
            let kind = suit.kind;
            let mut chis = Vec::new();
            let chi = |(a, b): (Num, Num)| {
                let face_a: TileFace = Suit { kind, num: a }.into();
                let face_b: TileFace = Suit { kind, num: b }.into();
                let idx_set_a = self.tiles.get_face(face_a);
                let idx_set_b = self.tiles.get_face(face_b);

                ChiOptions {
                    claim,
                    tiles: [(face_a, idx_set_a), (face_b, idx_set_b)],
                }
            };
            if let Some((a, b)) = num.next_two() {
                if self.tiles.has_face(Suit { kind, num: a }.into())
                    && self.tiles.has_face(Suit { kind, num: b }.into())
                {
                    chis.push(chi((a, b)));
                }
            }
            if let Some((a, b)) = num.prev_two() {
                if self.tiles.has_face(Suit { kind, num: a }.into())
                    && self.tiles.has_face(Suit { kind, num: b }.into())
                {
                    chis.push(chi((a, b)));
                }
            }
            if let Some((a, b)) = num.prev_and_next() {
                if self.tiles.has_face(Suit { kind, num: a }.into())
                    && self.tiles.has_face(Suit { kind, num: b }.into())
                {
                    chis.push(chi((a, b)));
                }
            }
            chis
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Melded {
    pub chi: Vec<Chi>,
    pub peng: Vec<Peng>,
    pub gang: Vec<Gang>,
}

impl Melded {
    pub fn new() -> Self {
        Melded {
            chi: Vec::new(),
            peng: Vec::new(),
            gang: Vec::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ChiOptions {
    pub claim: TileId,
    pub tiles: [(TileFace, TileIndexSet); 2],
}
#[derive(Debug, Clone, Copy)]

pub struct Peng {
    pub claim: Discard,
    pub tiles: [TileId; 2],
}

#[derive(Debug, Clone, Copy)]
pub enum Gang {
    AnGang { displayed: TileId },
    MingGang { claim: TileId },
    JiaGang { peng: Peng, claim: TileId },
}

#[derive(Debug, Clone, Copy)]

pub struct Chi {
    pub claim: Discard,
    pub tiles: [TileId; 2],
}
#[derive(Debug, Default, Clone)]
pub struct Deck {
    pub hand: Hand,
    pub melded: Melded,
}

impl Deck {
    pub fn chi(&mut self, chi: Chi) -> Result<(), Chi> {
        if !self.hand.tiles.has(chi.tiles[0]) || !self.hand.tiles.has(chi.tiles[1]) {
            return Err(chi);
        }

        self.hand.tiles.remove(chi.tiles[0]);
        self.hand.tiles.remove(chi.tiles[1]);
        self.melded.chi.push(chi);
        Ok(())
    }
    pub fn gang(&mut self, gang: Gang) -> Result<(), Gang> {
        todo!()
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tile in self.tiles.iter() {
            write!(f, "{}", tile.face().unicode())?;
        }
        Ok(())
    }
}
