#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Wind {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

impl Wind {
    pub const fn from_index(idx: usize) -> Self {
        match idx % 4 {
            0 => Wind::East,
            1 => Wind::South,
            2 => Wind::West,
            3 => Wind::North,
            _ => unreachable!(),
        }
    }
    pub const fn as_index(&self) -> usize {
        match self {
            Wind::East => 0,
            Wind::South => 1,
            Wind::West => 2,
            Wind::North => 3,
        }
    }
    pub fn enumerate() -> <[Self; 4] as IntoIterator>::IntoIter {
        [Wind::East, Wind::South, Wind::West, Wind::North].into_iter()
    }
    pub fn iter_from(self) -> impl Iterator<Item = Wind> + Clone {
        match self {
            Wind::East => [Wind::East, Wind::South, Wind::West, Wind::North].into_iter(),
            Wind::South => [Wind::South, Wind::West, Wind::North, Wind::East].into_iter(),
            Wind::West => [Wind::West, Wind::North, Wind::East, Wind::South].into_iter(),
            Wind::North => [Wind::North, Wind::East, Wind::South, Wind::West].into_iter(),
        }
    }
    pub const fn unicode(self) -> char {
        match self {
            Wind::East => '🀀',
            Wind::South => '🀁',
            Wind::West => '🀂',
            Wind::North => '🀃',
        }
    }
    pub const fn next(self) -> Self {
        match self {
            Wind::East => Wind::South,
            Wind::South => Wind::West,
            Wind::West => Wind::North,
            Wind::North => Wind::East,
        }
    }
}
