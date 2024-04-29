use crate::tile::Wind;

use super::player::{Discard, Player, PlayerObject};

pub struct Round {
    wind: Wind,
    player: [Box<dyn PlayerObject>; 4],
}

impl Round {
    pub fn new<P1, P2, P3, P4>(p1: P1, p2: P2, p3: P3, p4: P4) -> Self
    where
        P1: Player + 'static,
        P2: Player + 'static,
        P3: Player + 'static,
        P4: Player + 'static,
    {
        Round {
            wind: Wind::East,
            player: [
                Box::new(p1) as Box<dyn PlayerObject>,
                Box::new(p2) as Box<dyn PlayerObject>,
                Box::new(p3) as Box<dyn PlayerObject>,
                Box::new(p4) as Box<dyn PlayerObject>,
            ],
        }
    }
    pub fn player_mut(&mut self, wind: Wind) -> &mut dyn PlayerObject {
        self.player[wind.as_index()].as_mut()
    }
}
