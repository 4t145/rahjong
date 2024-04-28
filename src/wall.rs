use std::usize;

use crate::{tile::TileId, Tiles};
use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
pub struct Wall<S, R> {
    pub tiles: Vec<Option<TileId>>,
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
    pub fn new(set: S, rng: R) -> Self {
        let tiles = set.new_wall();
        Wall {
            tiles: tiles.into_iter().map(Some).collect(),
            _set: set,
            rng,
        }
    }
    pub fn draw_next(&mut self) -> Option<TileId> {
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
        self.tiles.get_mut(n)?.take()
    }
    pub fn take_last(&mut self) -> Option<TileId> {
        self.tiles.last_mut().and_then(Option::take)
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
