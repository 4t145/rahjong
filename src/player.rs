use crate::{
    discard::Discard,
    tile::{TileId, Wind},
};
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Player {
    wind: Wind,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct WindSet<T> {
    set: [T; 4],
}
impl<T> WindSet<T> {
    pub fn iter(&self) -> impl Iterator<Item = (Player, &T)> {
        self.set
            .iter()
            .enumerate()
            .map(|(i, t)| (Player::from(Wind::from_index(i)), t))
    }
    pub fn get<W: Into<Wind>>(&self, wind: W) -> &T {
        &self.set[wind.into() as usize]
    }
    pub fn get_mut<W: Into<Wind>>(&mut self, wind: W) -> &mut T {
        &mut self.set[wind.into() as usize]
    }
}

impl Player {
    pub fn wind(&self) -> Wind {
        self.wind
    }
    pub fn discard(&self, tile: TileId) -> Discard {
        Discard::new(*self, tile)
    }
    pub fn next(&self) -> Player {
        Player {
            wind: self.wind.next(),
        }
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
