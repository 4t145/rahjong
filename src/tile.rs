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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct TileFace(pub(crate) u8);
impl TileFace {
    fn is_suit(&self) -> bool {
        
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct TileIndex(pub(crate) u8);
const START: char = '\u{1F000}';

impl From<char> for TileFace {
    fn from(c: char) -> Self {
        let face = c as u32 - START as u32;
        TileFace(face as u8)
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
    Dot,
    Bamboo,
    Character,
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
