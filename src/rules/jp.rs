use rand::{seq::SliceRandom, Rng, SeedableRng};
pub mod player;
pub mod round;
pub mod riichi;
pub mod win;
use crate::{
    hand::{Chi, Deck, Gang, Hand, Peng},
    tile::{
        self, tile_set::TileSet, Dragon, Honer, Num, Suit, SuitKind, TileFace, TileId, TileIndex,
        Wind, B5, C5, D5,
    },
    wall::Wall,
    Tiles,
};
#[derive(Debug)]
pub struct Jp;
#[test]
fn build_jp_tiles() {
    let mut tiles = Vec::with_capacity(136);
    for suit in SuitKind::enumerate() {
        for num in Num::enumerate() {
            let face: TileFace = Suit { kind: suit, num }.into();
            for index in TileIndex::enumerate() {
                tiles.push(TileId::from_face_idx(face, index));
            }
        }
    }
    for honer in Honer::enumerate() {
        let face: TileFace = honer.into();
        for index in TileIndex::enumerate() {
            tiles.push(TileId::from_face_idx(face, index));
        }
    }
    let bytes: Vec<u8> = tiles.into_iter().map(TileId::into_inner).collect();
    std::fs::write("./src/rules/jp/tiles", bytes).expect("Failed to write file");
}

pub const JP_TILES: [TileId; 136] = unsafe { std::mem::transmute_copy(include_bytes!("jp/tiles")) };
pub const RED_B5: TileId = TileId::from_face_idx(B5, TileIndex::T0);
pub const RED_C5: TileId = TileId::from_face_idx(C5, TileIndex::T0);
pub const RED_D5: TileId = TileId::from_face_idx(D5, TileIndex::T0);
impl Tiles for Jp {
    fn new_wall(&self) -> Vec<TileId> {
        JP_TILES.to_vec()
    }

    fn size(&self) -> usize {
        136
    }
}
impl TileFace {
    pub fn is_green(self) -> bool {
        self == tile::GREEN
            || self == tile::B2
            || self == tile::B3
            || self == tile::B4
            || self == tile::B6
            || self == tile::B8
    }
}

impl TileId {
    pub fn is_red_dora(self) -> bool {
        self == RED_B5 || self == RED_C5 || self == RED_D5
    }
}

pub struct PlayerInfo {
    pub score: u32,
    pub wind: Wind,
}

pub struct Game4P<R> {
    pub players: [PlayerInfo; 4],
    pub wall: Wall<Jp, R>,
}
#[derive(Debug)]
pub struct DoraSet {
    indicator: TileId,
    ura: [TileId; 5],
    kan: [TileId; 4],
    rinshan: [TileId; 4],
    kan_index: usize,
}

impl DoraSet {
    pub fn indicator(&self) -> TileId {
        self.ura[0]
    }
    pub fn kan(&mut self) -> Option<TileId> {
        if self.kan_index < 4 {
            let tile = self.rinshan[self.kan_index];
            self.kan_index += 1;
            Some(tile)
        } else {
            None
        }
    }
    pub fn is_kan_dora_indicator(&self, tile: TileId) -> bool {
        self.kan[0..self.kan_index].contains(&tile)
    }
    pub fn is_inner_dora_indicator(&self, tile: TileId) -> bool {
        self.ura.contains(&tile)
    }
    pub fn is_dora_indicator(&self, tile: TileId) -> bool {
        self.indicator == tile
    }
}

impl<R> Wall<Jp, R> {
    /// Panics if the wall has less than 14 tiles.
    pub fn take_doras(&mut self) -> DoraSet {
        let doras = self.take_n::<14>().expect("wall has less than 14 tiles");
        let inner = [doras[0], doras[1], doras[2], doras[3], doras[4]];
        let indicator = doras[5];
        let outer = [doras[6], doras[7], doras[8], doras[9]];
        let rinshan = [doras[10], doras[11], doras[12], doras[13]];
        DoraSet {
            ura: inner,
            kan: outer,
            rinshan,
            indicator,
            kan_index: 0,
        }
    }
}


impl Deck {
    // pub fn can_win(&self, tile: TileId) -> bool {

    // }
}

#[derive(Debug)]
pub struct GameRound<R> {
    pub wind: Wind,
    pub dealer: Wind,
    pub number: usize,
    pub wall: Wall<Jp, R>,
    pub decks: [Deck; 4],
    pub dora_set: DoraSet,
}
impl<R> GameRound<R>
where
    R: Rng,
{
    pub fn new(wind: Wind, dealer: Wind, number: usize, rng: R) -> Self {
        let mut wall = Wall::new(Jp, rng);
        wall.shuffle();
        let hands = wall.draw_init::<4, 13>();
        let dora_set = wall.take_doras();
        let decks = [
            Deck {
                hand: Hand::new(hands[0]),
                melded: Default::default(),
            },
            Deck {
                hand: Hand::new(hands[1]),
                melded: Default::default(),
            },
            Deck {
                hand: Hand::new(hands[2]),
                melded: Default::default(),
            },
            Deck {
                hand: Hand::new(hands[3]),
                melded: Default::default(),
            },
        ];
        GameRound {
            wind,
            dealer,
            wall,
            decks,
            number,
            dora_set,
        }
    }
    pub fn deck(&self, wind: Wind) -> &Deck {
        &self.decks[wind.as_index()]
    }
    pub fn operations(&self, player: Wind) -> Vec<Operation> {
        vec![]
    }
    pub fn apply(&mut self, player: Wind, operation: Operation) -> Result<(), OperationError> {
        let deck = &mut self.decks[player.as_index()];
        match operation {
            Operation::Chi(chi) => deck.chi(chi).map_err(|_| OperationError::MissingTile),

            _ => {
                todo!()
            }
        }
    }
}

pub enum Operation {
    Chi(Chi),
    Pon(Peng),
    Kan(Gang),
    Ron(TileId),
    Tsumo,
    Draw,
}
pub enum OperationError {
    MissingTile,
    Unable,
}

pub enum PlayerState {
    DrawAndDiscard(TileId),
    End,
    OthersTurn,
}



#[test]
fn test_new_round() {
    let rng = rand::thread_rng();
    let round = GameRound::new(Wind::East, Wind::East, 1, rng);
    dbg!(round);
}
