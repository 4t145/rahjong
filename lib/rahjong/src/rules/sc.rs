use super::Rule;
use crate::{
    hand::{Deck, Melded},
    tile::{Num, Suit, SuitKind, TileFace, TileId},
};

impl Default for TileId {
    fn default() -> Self {
        TileId::from_face_idx(TileFace::from_suit(Suit {
            kind: SuitKind::Character,
            num: Num::N1,
        }), crate::tile::TileIndex::T0)
    }
}

pub struct SichuanRules;

impl Rule for SichuanRules {
    fn check_win(deck: &Deck, tile: TileId) -> bool {
        // 四川麻将和牌规则：
        // 1. 必须缺一门
        // 2. 基本和牌型：4副刻子或顺子 + 1对将
        // 3. 特殊牌型：七对、龙七对等

        // 检查是否缺一门
        if !Self::is_missing_one_suit(deck) {
            return false;
        }

        // 检查基本和牌型
        if Self::is_basic_win(deck) {
            return true;
        }

        // 检查特殊牌型
        Self::is_special_win(deck)
    }
}

impl SichuanRules {
    // 检查是否缺一门
    fn is_missing_one_suit(deck: &Deck) -> bool {
        use crate::tile::Suit;

        let mut suits = [false; 3]; // 万、条、筒
        for tile in deck.hand.tiles.iter() {
            if let Some(suit) = tile.face().try_into_suit() {
                match suit.kind {
                    SuitKind::Character => suits[0] = true,
                    SuitKind::Bamboo => suits[1] = true,
                    SuitKind::Dot => suits[2] = true,
                    _ => (),
                }
            }
        }

        // 统计缺少的花色数量
        let missing_count = suits.iter().filter(|&&has| !has).count();
        missing_count == 1
    }

    // 检查基本和牌型
    fn is_basic_win(deck: &Deck) -> bool {
        use crate::tile::{TileFace, TileId};

        // 将TileSet转换为Vec<TileId>以便排序
        let mut tiles: Vec<TileId> = deck.hand.tiles.iter().collect();
        tiles.sort();

        // 查找将牌
        let mut pair_count = 0;
        let mut i = 0;
        while i + 1 < tiles.len() {
            if tiles[i] == tiles[i + 1] {
                pair_count += 1;
                i += 2;
            } else {
                i += 1;
            }
        }

        // 必须有且只有一对将牌
        if pair_count != 1 {
            return false;
        }

        // 检查剩余牌是否能组成4副刻子或顺子
        let mut remaining_tiles = tiles.clone();
        Self::remove_pairs(&mut remaining_tiles);

        Self::can_form_melds(&remaining_tiles)
    }

    // 移除所有对子
    fn remove_pairs(tiles: &mut Vec<TileId>) {
        let mut i = 0;
        while i + 1 < tiles.len() {
            if tiles[i] == tiles[i + 1] {
                tiles.remove(i);
                tiles.remove(i);
            } else {
                i += 1;
            }
        }
    }

    // 检查是否能组成刻子或顺子
    fn can_form_melds(tiles: &[TileId]) -> bool {
        if tiles.is_empty() {
            return true;
        }

        // 尝试组成刻子
        if tiles.len() >= 3
            && tiles[0] == tiles[1]
            && tiles[1] == tiles[2]
            && Self::can_form_melds(&tiles[3..])
        {
            return true;
        }

        // 尝试组成顺子
        if let Some((second, third)) = Self::find_consecutive_tiles(tiles) {
            let mut remaining = tiles.to_vec();
            remaining.remove(0);
            remaining.remove(remaining.iter().position(|&x| x == second).unwrap());
            remaining.remove(remaining.iter().position(|&x| x == third).unwrap());

            if Self::can_form_melds(&remaining) {
                return true;
            }
        }

        false
    }

    // 查找连续牌
    fn find_consecutive_tiles(tiles: &[TileId]) -> Option<(TileId, TileId)> {
        if tiles.len() < 3 {
            return None;
        }

        let first = tiles[0];
        if let Some(suit) = first.face().try_into_suit() {
            if suit.num == Num::N8 || suit.num == Num::N9 {
                return None;
            }

            let second = TileId::from_face_idx(
                TileFace::from_suit(Suit {
                    kind: suit.kind,
                    num: suit.num.next()?,
                }),
                first.into_face_idx().1,
            );

            let third = TileId::from_face_idx(
                TileFace::from_suit(Suit {
                    kind: suit.kind,
                    num: suit.num.next()?.next()?,
                }),
                first.into_face_idx().1,
            );

            if tiles.contains(&second) && tiles.contains(&third) {
                return Some((second, third));
            }
        }

        None
    }

    // 检查特殊牌型
    fn is_special_win(deck: &Deck) -> bool {
        // 七对：7个对子
        if Self::is_seven_pairs(deck) {
            return true;
        }

        // 龙七对：7个对子，其中有一个四张相同的牌
        if Self::is_dragon_seven_pairs(deck) {
            return true;
        }

        false
    }

    // 检查七对
    fn is_seven_pairs(deck: &Deck) -> bool {
        let mut tiles: Vec<TileId> = deck.hand.tiles.iter().collect();
        tiles.sort();

        let mut pair_count = 0;
        let mut i = 0;
        while i + 1 < tiles.len() {
            if tiles[i] == tiles[i + 1] {
                pair_count += 1;
                i += 2;
            } else {
                i += 1;
            }
        }

        pair_count == 7
    }

    // 检查龙七对
    fn is_dragon_seven_pairs(deck: &Deck) -> bool {
        let mut tiles: Vec<TileId> = deck.hand.tiles.iter().collect();
        tiles.sort();

        let mut pair_count = 0;
        let mut has_quad = false;
        let mut i = 0;

        while i + 1 < tiles.len() {
            if tiles[i] == tiles[i + 1] {
                pair_count += 1;
                // 检查是否有四张相同的牌
                if i + 3 < tiles.len() && tiles[i] == tiles[i + 3] {
                    has_quad = true;
                    i += 4;
                } else {
                    i += 2;
                }
            } else {
                i += 1;
            }
        }

        pair_count == 7 && has_quad
    }

    // 四川麻将特有规则
    pub fn blood_battle(&self, deck: &Deck) -> bool {
        // 血战到底规则：当有玩家和牌后，其他玩家继续游戏直到只剩一个玩家
        // 检查当前玩家是否已经和牌
        Self::check_win(deck, TileId::default())
    }

    pub fn wind_rain(&self, deck: &Deck, tile: TileId) -> bool {
        // 刮风下雨规则：杠牌后立即结算
        // 检查是否有杠牌
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::{Deck, Hand, Melded};
    use crate::tile::{Num, Suit, SuitKind, TileFace, TileId};

    fn create_tile(suit: SuitKind, num: u8) -> TileId {
        TileId::from_face_idx(
            TileFace::from_suit(Suit {
                kind: suit,
                num: Num::try_from_u8(num).unwrap(),
            }),
            crate::tile::TileIndex::T0,
        )
    }

    #[test]
    fn test_seven_pairs() {
        let mut tiles = Vec::new();

        // 创建7个对子
        for i in 0..7 {
            let tile = create_tile(SuitKind::Character, i % 9 + 1);
            tiles.push(tile);
            tiles.push(tile);
        }

        let deck = Deck {
            hand: Hand::new(tiles),
            melded: Melded::new(),
        };
        assert!(SichuanRules::is_seven_pairs(&deck));
    }

    #[test]
    fn test_dragon_seven_pairs() {
        let mut tiles = Vec::new();

        // 创建6个对子
        for i in 0..6 {
            let tile = create_tile(SuitKind::Character, i % 9 + 1);
            tiles.push(tile);
            tiles.push(tile);
        }

        // 添加一个四张相同的牌
        let quad_tile = create_tile(SuitKind::Bamboo, 1);
        for _ in 0..4 {
            tiles.push(quad_tile);
        }

        let deck = Deck {
            hand: Hand::new(tiles),
            melded: Melded::new(),
        };
        assert!(SichuanRules::is_dragon_seven_pairs(&deck));
    }
}
