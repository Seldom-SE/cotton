use bevy::prelude::*;

use crate::{
    board::{
        Board, BoardIndex, BUILDING_BUILDING_ADJACENCY, BUILDING_ROAD_ADJACENCY,
        BUILDING_TILE_ADJACENCY,
    },
    button::{BuildingButton, Clicked},
    color::PlayerColor,
    image::UpdateImages,
    resource::{Hands, Resource},
    road::RoadSlot,
    tile::Tile,
    turn::{Players, Turn},
    ui::BuildSettlementButton,
};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_building_buttons)
            .add_system(build_settlement);
    }
}

#[derive(Clone, Copy)]
pub enum BuildingType {
    Settlement,
    City,
}

impl BuildingType {
    /// Multiplier on resource production
    pub fn production(self) -> u8 {
        match self {
            Self::Settlement => 1,
            Self::City => 2,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Building {
    pub building_type: BuildingType,
    pub color: PlayerColor,
}

#[derive(Clone, Component, Copy, Deref, DerefMut)]
pub struct BuildingSlot(pub Option<Building>);

impl UpdateImages for BuildingSlot {
    fn image(self, _: usize) -> Option<&'static str> {
        match *self {
            None => None,
            Some(Building {
                building_type: BuildingType::Settlement,
                color,
            }) => Some(match color {
                PlayerColor::Blue => "blue_settlement.png",
                PlayerColor::Orange => "orange_settlement.png",
                PlayerColor::Red => "red_settlement.png",
                PlayerColor::White => "white_settlement.png",
            }),
            Some(Building {
                building_type: BuildingType::City,
                color,
            }) => Some(match color {
                PlayerColor::Blue => "blue_city.png",
                PlayerColor::Orange => "orange_city.png",
                PlayerColor::Red => "red_city.png",
                PlayerColor::White => "white_city.png",
            }),
        }
    }
}

/// Show the buttons that appear when building settlements or cities
pub fn show_building_buttons(
    mut buttons: Query<(&mut Visibility, &BoardIndex), With<BuildingButton>>,
    // This query is for the UI button that enters build mode
    build_buttons: Query<&Interaction, (With<BuildSettlementButton>, Changed<Interaction>)>,
    buildings: Query<&BuildingSlot>,
    roads: Query<&RoadSlot>,
    board: Res<Board>,
    players: Res<Players>,
    mut hands: ResMut<Hands>,
    mut turn: ResMut<Turn>,
) {
    if let Some((player, setup)) = match *turn {
        // We're in a settlement-building phase of a setup round
        // Show the buttons if we've just entered the phase
        Turn::Setup {
            player,
            road: false,
            ..
        } => turn.is_changed().then(|| (player, true)),
        // We're in a build phase
        // Show the buttons if the player pressed the button to build, and has enough resources
        Turn::Build { player } => build_buttons.get_single().ok().and_then(|interaction| {
            if let Interaction::Clicked = interaction {
                let hand = hands[players[player] as usize];

                (hand[Resource::Brick as usize] >= 1
                    && hand[Resource::Wool as usize] >= 1
                    && hand[Resource::Grain as usize] >= 1
                    && hand[Resource::Lumber as usize] >= 1)
                    .then(|| (player, false))
            } else {
                None
            }
        }),
        _ => None,
    } {
        let mut can_build = false;
        let color = players[player];

        for (mut visibility, index) in buttons.iter_mut() {
            // The player may build a settlement here if it's next to that player's road,
            // there are no buildings here, and there are no adjacent roads.
            // The first criterion is relaxed in the setup phase.
            let visible = (setup
                || BUILDING_ROAD_ADJACENCY[**index].iter().any(|road| {
                    roads
                        .get(board.roads[*road])
                        .unwrap()
                        .map_or(false, |road| color == road.color)
                }))
                && buildings.get(board.buildings[**index]).unwrap().is_none()
                && BUILDING_BUILDING_ADJACENCY[**index]
                    .iter()
                    .all(|building| buildings.get(board.buildings[*building]).unwrap().is_none());

            visibility.is_visible = visible;
            can_build |= visible;
        }

        // If they aren't in setup, they should be charged for the build
        if can_build && !setup {
            let hand = &mut hands[color as usize];

            hand[Resource::Brick as usize] -= 1;
            hand[Resource::Wool as usize] -= 1;
            hand[Resource::Grain as usize] -= 1;
            hand[Resource::Lumber as usize] -= 1;

            *turn = Turn::BuildSettlement { player };
        }
    }
}

/// Build a settlement
fn build_settlement(
    mut commands: Commands,
    clicked_buttons: Query<(Entity, &BoardIndex), (With<BuildingButton>, With<Clicked>)>,
    mut buttons: Query<&mut Visibility, With<BuildingButton>>,
    mut buildings: Query<&mut BuildingSlot>,
    tiles: Query<&Tile>,
    board: Res<Board>,
    players: Res<Players>,
    mut turn: ResMut<Turn>,
    mut hands: ResMut<Hands>,
) {
    if let Some((round_2, player)) = match *turn {
        // We're in a settlement-building phase of a setup round
        Turn::Setup {
            round_2,
            player,
            road: false,
        } => Some((round_2, player)),
        // We're building a settlement because the player pressed the Build settlement button
        Turn::BuildSettlement { player } => Some((false, player)),
        _ => None,
    } {
        for (entity, index) in clicked_buttons.iter() {
            let color = players[player];

            commands.entity(entity).remove::<Clicked>();

            **buildings.get_mut(board.buildings[**index]).unwrap() = Some(Building {
                building_type: BuildingType::Settlement,
                color,
            });

            // In round 2 of setup, the player is given resources based on the tiles they started adjacent to
            if round_2 {
                let hand = &mut hands[color as usize];
                for tile in BUILDING_TILE_ADJACENCY[**index] {
                    if let Some(resource) = tiles.get(board.tiles[*tile]).unwrap().resource() {
                        hand[resource as usize] += 1;
                    }
                }
            }

            for mut visibility in buttons.iter_mut() {
                visibility.is_visible = false;
            }

            *turn = turn.next(*players);
        }
    }
}
