use std::borrow::Cow;

use rand::rngs::ThreadRng;

use crate::{
    discard::{Discard, DiscardSet},
    draw::Draw,
    hand::{Chi, Deck, Gang, Hand, Melded, Peng},
    player::{Player, WindSet},
    tile::{TileId, Wind},
    wall::Wall,
};

use super::{
    riichi::Richii,
    win::{Chankan, Ron, Tsumo, Win},
    DoraSet, Jp,
};

pub struct Round {
    dealer: Wind,
    wall: Wall<Jp, ThreadRng>,
    decks: WindSet<Deck>,
    discards: DiscardSet,
    dora_set: Option<DoraSet>,
    riichi: WindSet<Option<Richii>>,
    draw: Option<Draw>,
    state: RoundState,
    action_history: Vec<Action>,
    reaction: WindSet<Option<Action>>,
}

#[derive(Debug, Clone)]
pub struct PlayerSight {
    pub dealer: Wind,
    pub wall_rest: usize,
    pub self_deck: Deck,
    pub other_hand_size: WindSet<u8>,
    pub other_hand_melded: WindSet<Melded>,
    pub discards: DiscardSet,
    pub dora_indicators: Vec<TileId>,
    pub riichi: WindSet<Option<Richii>>,
    pub draw: Option<Draw>,
    pub to_discard: Option<Player>,
}

impl Round {
    pub fn player_sight(&self, player: Player) -> PlayerSight {
        let dealer = self.dealer;
        let wall_rest = self.wall.len();
        let self_deck = self.decks.get(player).clone();
        let other_hand_size = self
            .decks
            .iter()
            .filter(|(p, _)| *p != player)
            .map(|(p, d)| (p, d.hand.len() as u8))
            .collect();
        let other_hand_melded = self
            .decks
            .iter()
            .filter(|(p, _)| *p != player)
            .map(|(p, d)| (p, d.melded.clone()))
            .collect();
        let discards = self.discards.clone();
        let dora_indicators = self
            .dora_set
            .as_ref()
            .map(|d| d.shown_indicators())
            .unwrap_or_default();
        let riichi = self.riichi.clone();
        let draw = self.draw;
        let to_discard = match &self.state {
            RoundState::WaitDiscard(p) => Some(*p),
            _ => None,
        };
        PlayerSight {
            dealer,
            wall_rest,
            self_deck,
            other_hand_size,
            other_hand_melded,
            discards,
            dora_indicators,
            riichi,
            draw,
            to_discard,
        }
    }
    pub fn new(dealer: Wind) -> Self {
        Self {
            dealer,
            wall: Wall::new(Jp, Default::default()),
            decks: Default::default(),
            discards: Default::default(),
            dora_set: None,
            riichi: Default::default(),
            draw: None,
            state: RoundState::Init,
            action_history: Default::default(),
            reaction: Default::default(),
        }
    }
    pub fn start(&mut self) -> Result<(), Reason> {
        self.wall.shuffle();
        let hands = self.wall.draw_init::<4, 13>();
        self.dora_set.replace(self.wall.take_doras());
        for (i, hand) in hands.into_iter().enumerate() {
            self.decks.insert(
                Wind::from_index(i),
                Deck {
                    hand: Hand::new(hand),
                    melded: Default::default(),
                },
            );
        }
        self.draw_next(self.dealer.into());
        Ok(())
    }
    pub fn clear_reaction(&mut self) {
        self.reaction = WindSet::default()
    }
    pub fn all_passed(&self) -> bool {
        self.reaction
            .iter()
            .all(|(_, p)| matches!(p, Some(Action::Pass)))
    }
    pub fn someone_pon(&self) -> Option<(Player, Peng)> {
        for (p, a) in self.reaction.iter() {
            if let Some(Action::Pon(pon)) = a {
                return Some((p, *pon));
            }
        }
        None
    }
    pub fn draw_next(&mut self, for_player: Player) {
        self.draw = self.wall.draw_next();
        if let Some(next) = self.draw {
            self.state = RoundState::WaitDiscard(for_player);
        } else {
            self.state = RoundState::End;
        }
    }
}
pub enum RoundState {
    Init,
    WaitDiscard(Player),
    WaitDiscardReaction(Discard),
    WaitGanReaction(Gang),
    End,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Action {
    Pass,
    Discard(Discard),
    Tsumo(Tsumo),
    Chankan(Chankan),
    Ron(Ron),
    Kan(Gang),
    Chi(Chi),
    Pon(Peng),
    Richii(Richii),
    Ryukyoku,
}

pub struct Reason {
    kind: ReasonKind,
    expr: Cow<'static, str>,
}
impl std::fmt::Debug for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reason")
            .field("kind", &self.kind)
            .field("expr", &self.expr)
            .finish()
    }
}

#[derive(Debug)]
pub enum ReasonKind {
    InvalidOperation = 0x1001,
}

impl Reason {
    pub fn invalid_operation(expr: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind: ReasonKind::InvalidOperation,
            expr: expr.into(),
        }
    }
}
impl Round {
    pub fn apply(&mut self, action: &Action, source: Player) -> Result<(), Reason> {
        match (&self.state, action) {
            (RoundState::WaitDiscard(p), Action::Discard(d)) => {
                if !(*p == d.source() && *p == source) {
                    return Err(Reason::invalid_operation("Not your turn now"));
                }
                if self.decks.get(*p).hand.contains(d.tile()) {
                    return Err(Reason::invalid_operation("You don't have this tile"));
                }
                if let Some(Action::Chi(chi)) = self.action_history.last() {
                    if chi.claim.tile() == d.tile() {
                        return Err(Reason::invalid_operation("Cannot discard the claimed tile"));
                    }
                }
                self.decks.get_mut(*p).hand.remove(d.tile());
                self.discards.add(*d);
                self.clear_reaction();
                self.reaction.get_mut(source).replace(Action::Pass);
                self.state = RoundState::WaitDiscardReaction(*d);
            }
            (RoundState::WaitDiscardReaction(d), Action::Pass) => {
                self.reaction.get_mut(source).replace(Action::Pass);
                if self.all_passed() {
                    let player = d.source();
                    self.draw_next(player.next());
                }
            }

            _ => {
                return Err(Reason::invalid_operation("Rule violation"));
            }
        }
        Ok(())
    }
}

#[test]
fn test_round() {
    let mut round = Round::new(Wind::East);
    round.start().unwrap();
    let player_set = round.player_sight(Player::EAST);
    // post player set
    // 
    dbg!(player_set);
}
