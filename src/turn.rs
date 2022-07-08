use bevy::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use crate::color::{Color, COLORS};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Turn>().init_resource::<Players>();
    }
}

const PLAYER_COUNT: usize = 4;

struct Players([Color; PLAYER_COUNT]);

impl Default for Players {
    fn default() -> Self {
        let mut players = COLORS.clone();
        players.shuffle(&mut thread_rng());
        Players(players)
    }
}

pub enum Turn {
    Setup {
        round_2: bool,
        player: usize,
        road: bool,
    },
    Done,
}

impl Default for Turn {
    fn default() -> Self {
        Self::Setup {
            round_2: false,
            player: 0,
            road: false,
        }
    }
}
