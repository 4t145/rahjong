use crate::{
    discard::Discard,
    tile::{TileId, Wind},
};
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Player {
    wind: Wind,
}
impl Player {
    pub const EAST: Player = Player { wind: Wind::East };
    pub const SOUTH: Player = Player { wind: Wind::South };
    pub const WEST: Player = Player { wind: Wind::West };
    pub const NORTH: Player = Player { wind: Wind::North };
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
    pub fn insert<W: Into<Wind>>(&mut self, wind: W, value: T) {
        self.set[wind.into() as usize] = value;
    }
}

impl<P, T> FromIterator<(P, T)> for WindSet<T>
where
    P: Into<Player>,
    T: Default,
{
    fn from_iter<I: IntoIterator<Item = (P, T)>>(iter: I) -> Self {
        let mut set: [T; 4] = Default::default();
        for (player, value) in iter {
            let player: Player = player.into();
            set[player.wind.as_index()] = value;
        }
        WindSet { set }
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



pub trait AsyncPlayer {
    async fn draw(&self, tile: TileId) -> Draw;
}