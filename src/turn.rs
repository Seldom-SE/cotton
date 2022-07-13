use bevy::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    color::{PlayerColor, COLORS},
    ui::NextButton,
};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Turn>()
            .init_resource::<Players>()
            .add_system(press_next_button);
    }
}

pub const PLAYER_COUNT: usize = 4;
const LAST_PLAYER: usize = PLAYER_COUNT - 1;

/// Represents the turn order
#[derive(Clone, Copy, Deref)]
pub struct Players([PlayerColor; PLAYER_COUNT]);

impl Default for Players {
    fn default() -> Self {
        let mut players = COLORS;
        players.shuffle(&mut thread_rng());
        Players(players)
    }
}

/// Represents what phase we're in
#[derive(Clone, Copy)]
pub enum Turn {
    Setup {
        round_2: bool,
        player: usize,
        road: bool,
    },
    Production {
        player: usize,
    },
    Build {
        player: usize,
    },
    BuildRoad {
        player: usize,
    },
    BuildSettlement {
        player: usize,
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
    /// Gets the default next `Turn`
    pub fn next(self, players: Players) -> Self {
        let turn = match self {
            // Done building a settlement in setup phase
            Self::Setup {
                round_2,
                player,
                road: false,
            } => Self::Setup {
                round_2,
                player,
                road: true,
            },
            // Done with round 1 of setup
            Self::Setup {
                round_2: false,
                player: LAST_PLAYER,
                road: true,
            } => Self::Setup {
                round_2: true,
                player: LAST_PLAYER,
                road: false,
            },
            // Player is done with their setup turn
            Self::Setup {
                round_2: false,
                player,
                road: true,
            } => Self::Setup {
                round_2: false,
                player: player + 1,
                road: false,
            },
            // Done with round 2 of setup
            Self::Setup {
                round_2: true,
                player: 0,
                road: true,
            }
            // Done with a full round of gameplay
            | Self::Build {
                player: LAST_PLAYER,
            } => Self::Production { player: 0 },
            // Player is done with their setup turn in round 2, which advances backwards
            Self::Setup {
                round_2: true,
                player,
                road: true,
            } => Self::Setup {
                round_2: true,
                player: player - 1,
                road: false,
            },
            // Done with production
            Self::Production { player } => Self::Build { player },
            // Done with build mode
            Self::Build { player } => Self::Production { player: player + 1 },
            // Finished building a road
            Self::BuildRoad { player } => Self::Build { player },
            // Finished building a settlement
            Self::BuildSettlement { player } => Self::Build { player },
            Self::Done => Self::Done,
        };

        // Temporary, until a status bar is added
        turn.print(players);

        turn
    }

    /// Temporary, until a status bar is added
    fn print(self, players: Players) {
        match self {
            Self::Setup {
                round_2,
                player,
                road,
            } => println!(
                "Setup round {}: {}, place a {}",
                if round_2 { 2 } else { 1 },
                String::from(players[player]),
                if road { "road" } else { "settlement" }
            ),
            Self::Production { player } => {
                println!("{}: Production", String::from(players[player]))
            }
            Self::Build { player } => println!("{}: Build", String::from(players[player])),
            Self::BuildRoad { player } => {
                println!("{}: Build a road", String::from(players[player]))
            }
            Self::BuildSettlement { player } => {
                println!("{}: Build a settlement", String::from(players[player]))
            }
            Self::Done => println!("Game over"),
        }
    }
}

/// If in a phase that allows the next button, if the button is pressed, advance the turn
fn press_next_button(
    buttons: Query<&Interaction, (With<NextButton>, Changed<Interaction>)>,
    players: Res<Players>,
    mut turn: ResMut<Turn>,
) {
    if let Turn::Build { .. } = *turn {
        for interaction in buttons.iter() {
            if let Interaction::Clicked = interaction {
                *turn = turn.next(*players);
            }
        }
    }
}
