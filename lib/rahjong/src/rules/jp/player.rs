use std::future::Future;

use futures_util::future::BoxFuture;

use crate::{hand::{Chi, Deck, Gang, Peng}, tile::TileId};
pub struct Error;
pub trait Player {
    fn draw_and_discard(&mut self) -> impl Future<Output = Discard> + Send + 'static;
    fn reaction(&mut self, discard: TileId) -> impl Future<Output = Reaction> + Send + 'static;
    fn deck(&self) -> &Deck;
}

pub trait PlayerObject {
    fn draw_and_discard(&mut self) -> BoxFuture<'static, Discard>;
    fn reaction(&mut self, discard: TileId) -> BoxFuture<'static, Reaction>;
    fn deck(&self) -> &Deck;
}
pub enum Discard {
    Kan(Gang),
    Tsumo(TileId),
    Draw,
    Discard(TileId),
}
pub enum Reaction {
    Chi(Chi),
    Pon(Peng),
    Kan(Gang),
    Ron {
        tile: TileId,
    },
}
impl<P> PlayerObject for P
where
    P: Player,
{
    fn draw_and_discard(&mut self) -> BoxFuture<'static, Discard> {
        Box::pin(Player::draw_and_discard(self))
    }
    fn reaction(&mut self, discard: TileId) -> BoxFuture<'static, Reaction> {
        Box::pin(Player::reaction(self, discard))
    }
    fn deck(&self) -> &Deck {
        Player::deck(self)
    }
}

pub struct FourPlayers<P1, P2, P3, P4> {
    pub p1: P1,
    pub p2: P2,
    pub p3: P3,
    pub p4: P4,
}
