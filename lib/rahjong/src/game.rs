use crate::{
    discard::{AsyncDiscardSet, Discard},
    draw::Draw,
    hand::Deck,
    tile::TileId,
    wall::AsyncWall,
};

pub enum WinTile {
    Draw(Draw),
    Discard(Discard),
    Gang(TileId),
}
pub trait AsyncRule {
    type Error: std::error::Error;
    type Win;
    fn check(
        &self,
        deck: &Deck,
        win_tile: WinTile,
    ) -> impl std::future::Future<Output = Result<Option<Self::Win>, Self::Error>> + Send + 'static;
}

pub trait AsyncGame<W, D>
where
    W: AsyncWall,
    D: AsyncDiscardSet,
{
    type Rule: AsyncRule;
    fn wall(&mut self) -> &mut W;
    fn discard_set(&mut self) -> &mut D;
    async fn round(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let hands = self.wall().draw_next_n::<13>().await?;

        Ok(())
    }
}



