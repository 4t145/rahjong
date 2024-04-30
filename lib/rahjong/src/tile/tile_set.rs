use super::{TileFace, TileId, TileIndex};

#[derive(Clone, Default)]
pub struct TileSet {
    memory: [u8; 32],
}

impl std::fmt::Debug for TileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

const SET_0_MASK: u8 = 0x0f;
const SET_1_MASK: u8 = 0xf0;
#[derive(Clone, Default, Copy)]
#[repr(transparent)]
pub struct TileIndexSet {
    set: u8,
}

impl std::fmt::Debug for TileIndexSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
impl TileIndexSet {
    pub const fn const_from_u8(set: u8) -> Self {
        TileIndexSet { set: set & 0x0f }
    }
    pub const fn is_empty(self) -> bool {
        self.set == 0
    }
    pub const fn count(self) -> usize {
        self.set.count_ones() as usize
    }
    pub const fn has_index(self, index: TileIndex) -> bool {
        self.set & index.const_into_u8() == 1
    }

    pub const fn iter(self) -> TileIndexIter {
        TileIndexIter {
            bits: 0x01 | self.set,
        }
    }
}

pub struct TileIndexIter {
    bits: u8,
}

impl Iterator for TileIndexIter {
    type Item = TileIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = unsafe { TileIndex::const_from_mask(self.bits & 0xf0) };
        let set = TileIndexSet::const_from_u8(self.bits);
        while let Some(next) = index.next() {
            if set.has_index(next) {
                return Some(next);
            }
            index = next;
        }
        None
    }
}
#[inline]
const fn into_seg_dig(bits: u8) -> (u8, u8) {
    let seg = bits >> 3;
    let dig = bits & 0b111;
    (seg, dig)
}

#[inline]
const fn from_seg_dig(seg: u8, dig: u8) -> u8 {
    seg << 3 | dig
}

//    seg  dig
// 0x00000_000
impl TileSet {
    pub const fn new() -> Self {
        Self { memory: [0; 32] }
    }
    pub const fn has_face(&self, face: TileFace) -> bool {
        let (seg, dig) = into_seg_dig(TileId::from_face_idx(face, TileIndex::T0).into_inner());
        // dig is 0b000 or 0b100
        if dig == 0 {
            self.memory[seg as usize] & SET_0_MASK != 0
        } else {
            self.memory[seg as usize] & SET_1_MASK != 0
        }
    }
    pub const fn get_face(&self, face: TileFace) -> TileIndexSet {
        let (seg, dig) = into_seg_dig(TileId::from_face_idx(face, TileIndex::T0).into_inner());
        // dig is 0b000 or 0b100
        if dig == 0 {
            TileIndexSet::const_from_u8(self.memory[seg as usize] & SET_0_MASK)
        } else {
            TileIndexSet::const_from_u8(self.memory[seg as usize] & SET_1_MASK)
        }
    }
    pub const fn count_face(&self, face: TileFace) -> usize {
        self.get_face(face).count()
    }
    pub fn remove_one_face(&mut self, face: TileFace) -> Option<TileIndex> {
        let (seg, dig) = into_seg_dig(TileId::from_face_idx(face, TileIndex::T0).into_inner());
        let dig_mask = if dig == 0 { SET_0_MASK } else { SET_1_MASK };
        const MASK_0: u8 = 0b0001_0001;
        const MASK_1: u8 = 0b0010_0010;
        const MASK_2: u8 = 0b0100_0100;
        const MASK_3: u8 = 0b1000_1000;
        if seg & (MASK_0 & dig_mask) != 0 {
            self.memory[seg as usize] &= MASK_0 & dig_mask;
            Some(TileIndex::T0)
        } else if seg & (MASK_1 & dig_mask) != 0 {
            self.memory[seg as usize] &= MASK_1 & dig_mask;
            Some(TileIndex::T1)
        } else if seg & (MASK_2 & dig_mask) != 0 {
            self.memory[seg as usize] &= MASK_2 & dig_mask;
            Some(TileIndex::T2)
        } else if seg & (MASK_3 & dig_mask) != 0 {
            self.memory[seg as usize] &= MASK_3 & dig_mask;
            Some(TileIndex::T3)
        } else {
            None
        }
    }
    pub fn remove_all_face(&mut self, face: TileFace) {
        let (seg, dig) = into_seg_dig(TileId::from_face_idx(face, TileIndex::T0).into_inner());
        self.memory[seg as usize] &= if dig == 0 { !SET_0_MASK } else { !SET_1_MASK }
    }
    pub const fn has(&self, tile: TileId) -> bool {
        let (seg, dig) = into_seg_dig(tile.into_inner());
        self.memory[seg as usize] & (1 << dig) != 0
    }
    pub fn insert(&mut self, tile: TileId) {
        let (seg, dig) = into_seg_dig(tile.into_inner());
        self.memory[seg as usize] |= 1 << dig
    }
    pub fn remove(&mut self, tile: TileId) {
        let (seg, dig) = into_seg_dig(tile.into_inner());
        self.memory[seg as usize] &= !(1 << dig)
    }

    pub fn iter(&self) -> Iter {
        Iter {
            set: self,
            seg_idx: 0,
            dig_musk: 0b0000_0001_u8,
        }
    }
    pub fn is_empty(&self) -> bool {
        unsafe {
            let view = std::mem::transmute::<_, &[u64; 4]>(&self.memory);
            for block in view {
                if *block != 0 {
                    return false;
                }
            }
            true
        }
    }
    pub fn len(&self) -> usize {
        unsafe {
            let mut count = 0;
            let view = std::mem::transmute::<_, &[u64; 4]>(&self.memory);
            for block in view {
                if *block != 0 {
                    count += block.count_ones();
                }
            }
            count as usize
        }
    }
}

impl<'a> IntoIterator for &'a TileSet {
    type Item = TileId;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a> {
    set: &'a TileSet,
    seg_idx: u8,
    dig_musk: u8,
}
impl<'a> Iter<'a> {
    fn current(&self) -> Option<TileId> {
        let seg = self.set.memory[self.seg_idx as usize];
        if seg == 0 {
            return None;
        }
        let dig = self.dig_musk.trailing_zeros() as u8;
        if seg & self.dig_musk != 0 {
            Some(TileId::from_inner(from_seg_dig(self.seg_idx, dig)))
        } else {
            None
        }
    }
    fn forward(&mut self) {
        let seg = self.set.memory[self.seg_idx as usize];
        if seg == 0 {
            self.seg_idx += 1;
            self.dig_musk = 0b0000_0001_u8;
        } else {
            self.dig_musk = self.dig_musk.overflowing_shl(1).0;
            if self.dig_musk == 0 {
                self.seg_idx += 1;
                self.dig_musk = 0b0000_0001_u8;
            }
        }
    }
    fn finished(&self) -> bool {
        self.seg_idx == 32
    }
}
impl FromIterator<TileId> for TileSet {
    fn from_iter<I: IntoIterator<Item = TileId>>(iter: I) -> Self {
        let mut set = Self::new();
        for tile in iter {
            set.insert(tile);
        }
        set
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = TileId;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.finished() {
                return None;
            }
            if let Some(tile) = self.current() {
                self.forward();
                return Some(tile);
            } else {
                self.forward();
            }
        }
    }
}

#[test]
fn test_tile_set() {
    use crate::tile::*;
    let tile_vec = [
        TileId::from_face_idx(C1, TileIndex::T0),
        TileId::from_face_idx(C5, TileIndex::T1),
        TileId::from_face_idx(C9, TileIndex::T2),
        TileId::from_face_idx(D1, TileIndex::T3),
        TileId::from_face_idx(D5, TileIndex::T0),
        TileId::from_face_idx(D8, TileIndex::T1),
        TileId::from_face_idx(EAST, TileIndex::T2),
        TileId::from_face_idx(SOUTH, TileIndex::T3),
        TileId::from_face_idx(WEST, TileIndex::T0),
        TileId::from_face_idx(NORTH, TileIndex::T1),
        TileId::from_face_idx(RED, TileIndex::T2),
        TileId::from_face_idx(GREEN, TileIndex::T3),
        TileId::from_face_idx(WHITE, TileIndex::T0),
    ];
    let mut tile_set: TileSet = tile_vec.into_iter().collect();
    dbg!(&tile_set);
    for tile in tile_vec.iter() {
        assert!(tile_set.has(*tile));
    }
    for tile in tile_vec.iter() {
        tile_set.remove(*tile);
        println!("{}", tile.unicode());
        assert!(!tile_set.has(*tile));
        assert!(tile_vec.contains(tile));
    }
}
