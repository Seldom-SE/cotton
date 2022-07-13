use bevy::prelude::*;

use crate::{
    turn::{Players, Turn},
    ui::StatusBar,
};

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_status);
    }
}

/// Update the text in the status bar
fn update_status(
    mut status_bars: Query<&mut Text, With<StatusBar>>,
    turn: Res<Turn>,
    players: Res<Players>,
) {
    if turn.is_changed() {
        for mut text in status_bars.iter_mut() {
            text.sections[0].value = match *turn {
                Turn::Setup {
                    round_2,
                    player,
                    road,
                } => format!(
                    "Setup round {}: {}: build a {}",
                    if round_2 { "2" } else { "1" },
                    String::from(players[player]),
                    if road { "road" } else { "settlement" }
                ),
                Turn::Production { player } | Turn::Build { player } => {
                    format!("{}: build and trade", String::from(players[player]))
                }
                Turn::BuildRoad { player } => {
                    format!("{}: build a road", String::from(players[player]))
                }
                Turn::BuildSettlement { player } => {
                    format!("{}: build a settlement", String::from(players[player]))
                }
                Turn::Done => "Game over".to_string(),
            }
        }
    }
}
