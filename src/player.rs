use crate::{
    discard::Discard,
    tile::{TileId, Wind},
};
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Player {
    wind: Wind,
}

impl Player {
    pub fn wind(&self) -> Wind {
        self.wind
    }
    pub fn discard(&self, tile: TileId) -> Discard {
        Discard::new(*self, tile)
    }
}

impl From<Wind> for Player {
    fn from(wind: Wind) -> Self {
        Player { wind }
    }
}

impl From<Player> for Wind {
    fn from(val: Player) -> Self {
        val.wind
    }
}
