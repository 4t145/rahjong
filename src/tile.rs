pub trait Unicode {
    fn unicode(&self) -> char;
}

///
///```text
/// 0x____000000____00
///       uc_bias   idx
///```
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TileId {
    uid: u8,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TileFace(pub(crate) u8);
impl TileFace {
    pub const fn const_from_char(c: char) -> Self {
        let face = c as u32 - START as u32;
        TileFace(face as u8)
    }

    pub const fn try_into_suit(self) -> Option<Suit> {
        match self {
            TileFace(0x00..=0x08) => Some(Suit {
                kind: SuitKind::Bamboo,
                num: Num::const_from_u8(self.0 + 0x01),
            }),
            TileFace(0x09..=0x11) => Some(Suit {
                kind: SuitKind::Character,
                num: Num::const_from_u8(self.0 - 0x09),
            }),
            TileFace(0x12..=0x1A) => Some(Suit {
                kind: SuitKind::Dot,
                num: Num::const_from_u8(self.0 - 0x12),
            }),
            _ => None,
        }
    }

    pub const fn try_into_honer(self) -> Option<Honer> {
        match self {
            EAST => Some(Honer::Wind(Wind::East)),
            SOUTH => Some(Honer::Wind(Wind::South)),
            WEST => Some(Honer::Wind(Wind::West)),
            NORTH => Some(Honer::Wind(Wind::North)),
            RED => Some(Honer::Dragon(Dragon::Red)),
            GREEN => Some(Honer::Dragon(Dragon::Green)),
            WHITE => Some(Honer::Dragon(Dragon::White)),
            _ => None,
        }
    }
}

macro_rules! const_tiles {
    (
        $(
            $name:ident: $face:literal
        )*
    ) => {
        $(
            pub const $name: TileFace = TileFace::const_from_char($face);
        )*
    };
}
const_tiles! {
    B1: '🀐'
    B2: '🀑'
    B3: '🀒'
    B4: '🀓'
    B5: '🀔'
    B6: '🀕'
    B7: '🀖'
    B8: '🀗'
    B9: '🀘'
    C1: '🀇'
    C2: '🀈'
    C3: '🀉'
    C4: '🀊'
    C5: '🀋'
    C6: '🀌'
    C7: '🀍'
    C8: '🀎'
    C9: '🀏'
    D1: '🀙'
    D2: '🀚'
    D3: '🀛'
    D4: '🀜'
    D5: '🀝'
    D6: '🀞'
    D7: '🀟'
    D8: '🀠'
    D9: '🀡'
    EAST: '🀀'
    SOUTH: '🀁'
    WEST: '🀂'
    NORTH: '🀃'
    RED: '🀄'
    GREEN: '🀅'
    WHITE: '🀆'
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct TileIndex(pub(crate) u8);
const START: char = '\u{1F000}';

impl From<char> for TileFace {
    fn from(c: char) -> Self {
        Self::const_from_char(c)
    }
}

impl From<TileFace> for char {
    fn from(val: TileFace) -> Self {
        unsafe { char::from_u32_unchecked(START as u32 + val.0 as u32) }
    }
}

impl TileId {
    pub const fn from_face_idx(face: TileFace, idx: TileIndex) -> Self {
        let uid = face.0 << 2 | idx.0;
        TileId { uid }
    }
    pub const fn into_face_idx(&self) -> (TileFace, TileIndex) {
        let face = (self.uid & 0b1111_1100) >> 2;
        let idx = self.uid & 0b0000_0011;
        (TileFace(face), TileIndex(idx))
    }
    pub fn unicode(&self) -> char {
        let (face, _) = self.into_face_idx();
        face.into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Num {
    N1 = 1,
    N2 = 2,
    N3 = 3,
    N4 = 4,
    N5 = 5,
    N6 = 6,
    N7 = 7,
    N8 = 8,
    N9 = 9,
}

impl Num {
    pub const fn const_from_u8(n: u8) -> Self {
        match n {
            1 => Num::N1,
            2 => Num::N2,
            3 => Num::N3,
            4 => Num::N4,
            5 => Num::N5,
            6 => Num::N6,
            7 => Num::N7,
            8 => Num::N8,
            9 => Num::N9,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Honer {
    Wind(Wind),
    Dragon(Dragon),
}

impl Unicode for Honer {
    fn unicode(&self) -> char {
        match self {
            Honer::Wind(w) => w.unicode(),
            Honer::Dragon(d) => d.unicode(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Suit {
    kind: SuitKind,
    num: Num,
}

impl Unicode for Suit {
    fn unicode(&self) -> char {
        unsafe { char::from_u32_unchecked(self.kind.unicode_start() as u32 + self.num as u32 - 1) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SuitKind {
    Bamboo,
    Character,
    Dot,
}

impl SuitKind {
    pub const fn unicode_start(&self) -> char {
        match self {
            SuitKind::Dot => '🀙',
            SuitKind::Bamboo => '🀐',
            SuitKind::Character => '🀇',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dragon {
    Red,
    Green,
    White,
}

impl Unicode for Dragon {
    fn unicode(&self) -> char {
        match self {
            Dragon::Red => '🀄',
            Dragon::Green => '🀅',
            Dragon::White => '🀆',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

impl Unicode for Wind {
    fn unicode(&self) -> char {
        match self {
            Wind::East => '🀀',
            Wind::South => '🀁',
            Wind::West => '🀂',
            Wind::North => '🀃',
        }
    }
}
