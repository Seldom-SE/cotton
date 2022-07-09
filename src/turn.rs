use bevy::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use crate::color::{PlayerColor, COLORS};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Turn>().init_resource::<Players>();
    }
}

pub const PLAYER_COUNT: usize = 4;
const LAST_PLAYER: usize = PLAYER_COUNT - 1;

#[derive(Deref)]
pub struct Players([PlayerColor; PLAYER_COUNT]);

impl Default for Players {
    fn default() -> Self {
        let mut players = COLORS;
        players.shuffle(&mut thread_rng());
        Players(players)
    }
}

#[derive(Clone, Copy)]
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

impl Turn {
    pub fn next(self) -> Self {
        match self {
            Self::Setup {
                round_2,
                player,
                road: false,
            } => Self::Setup {
                round_2,
                player,
                road: true,
            },
            Self::Setup {
                round_2: false,
                player: LAST_PLAYER,
                road: true,
            } => Self::Setup {
                round_2: true,
                player: LAST_PLAYER,
                road: false,
            },
            Self::Setup {
                round_2: false,
                player,
                road: true,
            } => Self::Setup {
                round_2: false,
                player: player + 1,
                road: false,
            },
            Self::Setup {
                round_2: true,
                player: 0,
                road: true,
            } => Self::Done,
            Self::Setup {
                round_2: true,
                player,
                road: true,
            } => Self::Setup {
                round_2: true,
                player: player - 1,
                road: false,
            },
            Self::Done => Self::Done,
        }
    }
}
