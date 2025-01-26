use std::collections::HashMap;

use crate::{
    player::{Player, PlayerId},
    wall::Wall,
    wind::Wind,
};

pub struct Table<S, R> {
    pub wall: Wall<S, R>,
    pub players: HashMap<PlayerId, Player>,
    pub winds: HashMap<PlayerId, Wind>,
}
