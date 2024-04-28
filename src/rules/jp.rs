use rand::{seq::SliceRandom, Rng, SeedableRng};

use crate::{
    hand::{Deck, Hand},
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

pub struct Player {
    pub score: u32,
    pub wind: Wind,
}

pub struct Game4P<R> {
    pub players: [Player; 4],
    pub wall: Wall<Jp, R>,
}

#[derive(Debug)]
pub struct GameRound<R> {
    pub wind: Wind,
    pub dealer: Wind,
    pub number: usize,
    pub wall: Wall<Jp, R>,
    pub decks: [Deck; 4],
}

impl<R> GameRound<R>
where
    R: Rng,
{
    pub fn new(wind: Wind, dealer: Wind, number: usize, rng: R) -> Self {
        let mut wall = Wall::new(Jp, rng);
        wall.shuffle();
        let hands = wall.draw_init::<4, 13>();
        let decks = [
            Deck {
                hand: Hand::new(hands[0]),
                melded_set: Default::default(),
            },
            Deck {
                hand: Hand::new(hands[1]),
                melded_set: Default::default(),
            },
            Deck {
                hand: Hand::new(hands[2]),
                melded_set: Default::default(),
            },
            Deck {
                hand: Hand::new(hands[3]),
                melded_set: Default::default(),
            },
        ];
        GameRound {
            wind,
            dealer,
            wall,
            decks,
            number,
        }
    }
    pub fn deck(&self, wind: Wind) -> &Deck {
        &self.decks[wind.as_index()]
    }
}

#[test]
fn test_new_round() {
    let rng = rand::thread_rng();
    let round = GameRound::new(Wind::East, Wind::East, 1, rng);
    for wind in Wind::enumerate() {
        println!("{wind:?}\t: {}", round.deck(wind).hand);
    }
}
