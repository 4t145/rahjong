use std::usize;

use crate::{tile::TileId, Tiles};
use rand::{seq::SliceRandom, Rng};

pub struct Wall<S, R> {
    pub tiles: Vec<Option<TileId>>,
    pub size: usize,
    rng: R,
    _set: S,
}
impl<S: Tiles, R> Wall<S, R>
where
    R: Rng,
{
    pub fn shuffle(&mut self) {
        let rng = &mut self.rng;
        self.tiles.shuffle(rng);
    }
}
impl<S: Tiles, R> Wall<S, R> {
    pub fn new(set: S, rng: R) -> Self {
        let tiles = set.new_wall();
        Wall {
            size: tiles.len(),
            tiles: tiles.into_iter().map(Some).collect(),
            _set: set,
            rng,
        }
    }
    pub fn draw_next(&mut self) -> Option<TileId> {
        self.size = self.size.saturating_sub(1);
        loop {
            if self.tiles.is_empty() {
                break None;
            }
            if let Some(next) = self.tiles.pop().flatten() {
                break Some(next);
            }
        }
    }
    pub fn take_nth(&mut self, n: usize) -> Option<TileId> {
        self.size = self.size.saturating_sub(1);
        self.tiles.get_mut(n)?.take()
    }
    pub fn take_last(&mut self) -> Option<TileId> {
        self.size = self.size.saturating_sub(1);
        self.tiles.last_mut().and_then(Option::take)
    }
    pub fn take_n<const N: usize>(&mut self) -> Option<[TileId; N]> {
        let mut result = [TileId::from_inner(0); N];
        let mut idx = 0;
        if self.size < N {
            return None;
        }
        self.size = self.size.saturating_sub(N);
        loop {
            if let Some(tile) = self.tiles.pop().flatten() {
                result[idx] = tile;
            }
            idx += 1;
            if idx == N {
                return Some(result);
            }
        }
    }
    pub fn draw_init<const P_COUNT: usize, const I_COUNT: usize>(
        &mut self,
    ) -> [[TileId; I_COUNT]; P_COUNT] {
        let mut hands = [[TileId::from_inner(0); I_COUNT]; P_COUNT];
        for i in 0..I_COUNT {
            for hand in &mut hands {
                if let Some(tile) = self.draw_next() {
                    hand[i] = tile;
                }
            }
        }
        hands
    }
}

impl<S, R> std::fmt::Debug for Wall<S, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wall").field("tiles", &self.tiles).finish()
    }
}

impl<S, R> std::fmt::Display for Wall<S, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tile in &self.tiles {
            if let Some(tile) = tile {
                write!(f, "{}", tile.face().unicode())?;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

pub trait AsyncWall {
    type Error: std::error::Error + 'static;
    fn draw_next(
        &mut self,
    ) -> impl std::future::Future<Output = Result<Option<TileId>, Self::Error>> + Send + 'static;
    fn draw_next_n<const N: usize>(
        &mut self,
    ) -> impl std::future::Future<Output = Result<[TileId; N], Self::Error>> + Send + 'static;
    fn get_size(
        &self,
    ) -> impl std::future::Future<Output = Result<usize, Self::Error>> + Send + 'static;
}
#[test]
fn test_game() {
    let mut wall = Wall::new(crate::rules::jp::Jp, rand::rngs::ThreadRng::default());
    println!("{wall}");
    wall.shuffle();
    println!("{wall}");
    let hands = wall.draw_init::<4, 13>();
    println!("{wall}");
    for hand in hands {
        print!("{}, ", crate::hand::Hand::new(hand));
    }
}
