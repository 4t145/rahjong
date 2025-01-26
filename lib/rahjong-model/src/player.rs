use std::collections::{HashMap, HashSet};

use crate::{discard::Discard, draw::Draw, error::MessageCode, tile::TileId, Error};
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum PlayerId {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
pub struct Player {
    pub id: PlayerId,
    pub hands: HashSet<TileId>,
    pub drawn: Option<Draw>,
    pub melded: HashMap<MeldedId, MeldedType>,
    pub discards: Vec<Discard>,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct MeldedId {
    pub index: u8,
    pub player: PlayerId,
}

/// Represents different types of melds in mahjong
#[derive(Debug, Clone, PartialEq)]
pub enum MeldedType {
    Chi {
        tiles: [TileId; 3],
        discard: Discard,
    },
    Pon {
        tiles: [TileId; 3],
        discard: Discard,
    },
    /// Exposed Kong (大明杠) - formed by taking another player's discard
    ExposedKong {
        tiles: [TileId; 4],
        discard: Discard,
    },
    /// Added Kong (加杠) - formed by adding fourth tile to an existing Pong
    AddedKong {
        added: TileId,
        original_pon: [TileId; 3],
    },
    /// Concealed Kong (暗杠) - formed with four identical tiles from initial deal or draws
    ConcealedKong { tiles: [TileId; 4] },
}

impl Player {
    pub fn discard(&mut self, tile: TileId) -> Result<Discard, Error> {
        if let Some(drawn) = self.drawn {
            if drawn.tile() == tile {
                self.drawn = None;
                Ok(Discard {
                    tile,
                    player: self.id,
                })
            } else {
                let removed = self.hands.remove(&tile);
                if !removed {
                    return Err(Error::status_conflict(MessageCode::TILE_NOT_IN_HAND));
                }
                self.drawn = None;
                self.hands.insert(drawn.tile());
                let discard = Discard {
                    tile,
                    player: self.id,
                };
                Ok(discard)
            }
        } else {
            Err(Error::status_conflict(MessageCode::NOT_IN_TURN_TO_DISCARD))
        }
    }
    pub fn draw(&mut self, draw: Draw) {
        self.drawn = Some(draw);
    }
    pub fn pon(&mut self, hands: [TileId; 2], discard: Discard) -> Result<(), Error> {
        if !self.hands.contains(&hands[0]) || !self.hands.contains(&hands[1]) {
            return Err(Error::status_conflict(MessageCode::TILE_NOT_IN_HAND));
        }
        self.hands.remove(&hands[0]);
        self.hands.remove(&hands[1]);
        self.melded.push(MeldedType::Pon {
            tiles: [hands[0], hands[1], discard.tile],
            discard,
        });
        Ok(())
    }
    pub fn chi(&mut self, hands: [TileId; 2], discard: Discard) -> Result<(), Error> {
        if !self.hands.contains(&hands[0]) || !self.hands.contains(&hands[1]) {
            return Err(Error::status_conflict(MessageCode::TILE_NOT_IN_HAND));
        }
        self.hands.remove(&hands[0]);
        self.hands.remove(&hands[1]);
        self.melded.push(MeldedType::Chi {
            tiles: [hands[0], hands[1], discard.tile],
            discard,
        });
        Ok(())
    }
    pub fn exposed_kong(&mut self, hands: [TileId; 3], discard: Discard) -> Result<(), Error> {
        if !self.hands.contains(&hands[0])
            || !self.hands.contains(&hands[1])
            || !self.hands.contains(&hands[2])
        {
            return Err(Error::status_conflict(MessageCode::TILE_NOT_IN_HAND));
        }
        self.hands.remove(&hands[0]);
        self.hands.remove(&hands[1]);
        self.hands.remove(&hands[2]);
        self.melded.push(MeldedType::ExposedKong {
            tiles: [hands[0], hands[1], hands[2], discard.tile],
            discard,
        });
        Ok(())
    }

    pub fn added_kong(&mut self, added: TileId, original_pon: [TileId; 3]) -> Result<(), Error> {
        if !self.hands.contains(&added) {
            return Err(Error::status_conflict(MessageCode::TILE_NOT_IN_HAND));
        }
        self.hands.remove(&added);
        self.self.melded.push(MeldedType::AddedKong {
            added,
            original_pon,
        });
        Ok(())
    }
}
