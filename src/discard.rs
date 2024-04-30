use crate::{player::Player, tile::TileId};

#[derive(Debug, Clone, Copy)]
pub struct Discard {
    source: Player,
    tile: TileId,
}

impl Discard {
    pub fn new(from: Player, tile: TileId) -> Self {
        Discard { source: from, tile }
    }
    pub fn source(&self) -> Player {
        self.source
    }
    pub fn tile(&self) -> TileId {
        self.tile
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiscardSet {
    discards: Vec<Discard>,
}

impl DiscardSet {
    pub fn new() -> Self {
        DiscardSet {
            discards: Vec::new(),
        }
    }
    pub fn add(&mut self, discard: Discard) {
        self.discards.push(discard);
    }
    pub fn iter(&self) -> impl Iterator<Item = &Discard> {
        self.discards.iter()
    }
}

pub trait AsyncDiscardSet {
    type Error: std::error::Error + 'static;
    fn discard(
        &mut self,
        discard: Discard,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send + 'static;

}
