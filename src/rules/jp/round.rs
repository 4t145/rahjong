use std::borrow::Cow;

use rand::rngs::ThreadRng;

use crate::{
    discard::{Discard, DiscardSet},
    draw::Draw,
    hand::{Chi, Deck, Gang, Peng},
    player::{Player, WindSet},
    tile::Wind,
    wall::Wall,
};

use super::{
    riichi::Richii,
    win::{ChanKan, Ron, Tsumo, Win},
    Jp,
};

pub struct Round {
    wind: Wind,
    wall: Wall<Jp, ThreadRng>,
    decks: WindSet<Deck>,
    discards: DiscardSet,
    draw: Draw,
    state: RoundState,
    action_history: Vec<Action>,
    reaction: WindSet<Option<Action>>,
}

impl Round {
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
        if let Some(next) = self.wall.draw_next() {
            self.draw = next;
            self.state = RoundState::WaitDiscard(for_player);
        } else {
            self.state = RoundState::End;
        }
    }
}
pub enum RoundState {
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
    ChanKan(ChanKan),
    Ron(Ron),
    Kan(Gang),
    Chi(Chi),
    Pon(Peng),
    Richii(Richii),
    Ryukyoku,
    Draw(Draw),
}

pub struct Reason {
    kind: ReasonKind,
    expr: Cow<'static, str>,
}

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
