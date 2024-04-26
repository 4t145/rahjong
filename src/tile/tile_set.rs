use super::{TileFace, TileId, TileIndex};

#[derive(Clone, Default)]
pub struct TileSet {
    memory: [u8; 32],
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
            self.memory[seg as usize] & 0b0000_1111 != 0
        } else {
            self.memory[seg as usize] & 0b1111_0000 != 0
        }
    }
    pub const fn count_face(&self, face: TileFace) -> usize {
        let (seg, dig) = into_seg_dig(TileId::from_face_idx(face, TileIndex::T0).into_inner());
        // dig is 0b000 or 0b100
        if dig == 0 {
            (self.memory[seg as usize] & 0b0000_1111).count_ones() as usize
        } else {
            (self.memory[seg as usize] & 0b1111_0000).count_ones() as usize
        }
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
            let seg = self.set.memory[self.seg_idx as usize];
            if seg == 0 {
                self.seg_idx += 1;
                self.dig_musk = 0b0000_0001_u8;
                if self.seg_idx == 32 {
                    return None;
                }
            } else {
                loop {
                    if seg & self.dig_musk != 0 {
                        return Some(TileId::from_inner(from_seg_dig(
                            self.seg_idx,
                            self.dig_musk.trailing_zeros() as u8,
                        )));
                    }
                    self.dig_musk <<= 1;
                    if self.dig_musk == 0 {
                        break;
                    }
                }
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
