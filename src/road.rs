use bevy::prelude::*;

use crate::{
    board::{
        Board, BoardIndex, BUILDING_ROAD_ADJACENCY, ROAD_BUILDING_ADJACENCY, ROAD_ORIENTATIONS,
        ROAD_ROAD_ADJACENCY,
    },
    building::BuildingSlot,
    button::{Clicked, RoadButton},
    color::PlayerColor,
    image::UpdateImages,
    resource::{Hands, Resource},
    turn::{Players, Turn},
    ui::BuildRoadButton,
};

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_road_buttons).add_system(build_road);
    }
}

/// Represents the direction a road faces
#[derive(Clone, Copy)]
pub enum RoadOrientation {
    /// The road connects the bottom left and top right, like an increasing line
    Inc,
    /// The road connects the top left and bottom right, like a decreasing line
    Dec,
    /// The road connects the top and bottom: a vertical line
    Vert,
}

#[derive(Clone, Copy)]
pub struct Road {
    pub color: PlayerColor,
}

#[derive(Clone, Component, Copy, Deref, DerefMut)]
pub struct RoadSlot(pub Option<Road>);

impl UpdateImages for RoadSlot {
    fn image(self, index: usize) -> Option<&'static str> {
        let orientation = ROAD_ORIENTATIONS[index];
        match *self {
            None => None,
            Some(Road {
                color: PlayerColor::Blue,
            }) => Some(match orientation {
                RoadOrientation::Inc => "blue_inc_road.png",
                RoadOrientation::Dec => "blue_dec_road.png",
                RoadOrientation::Vert => "blue_vert_road.png",
            }),
            Some(Road {
                color: PlayerColor::Orange,
            }) => Some(match orientation {
                RoadOrientation::Inc => "orange_inc_road.png",
                RoadOrientation::Dec => "orange_dec_road.png",
                RoadOrientation::Vert => "orange_vert_road.png",
            }),
            Some(Road {
                color: PlayerColor::Red,
            }) => Some(match orientation {
                RoadOrientation::Inc => "red_inc_road.png",
                RoadOrientation::Dec => "red_dec_road.png",
                RoadOrientation::Vert => "red_vert_road.png",
            }),
            Some(Road {
                color: PlayerColor::White,
            }) => Some(match orientation {
                RoadOrientation::Inc => "white_inc_road.png",
                RoadOrientation::Dec => "white_dec_road.png",
                RoadOrientation::Vert => "white_vert_road.png",
            }),
        }
    }
}

/// Show the buttons that appear when building roads
pub fn show_road_buttons(
    mut buttons: Query<(&mut Visibility, &BoardIndex), With<RoadButton>>,
    // This query is for the UI button that enters build mode
    build_buttons: Query<&Interaction, (With<BuildRoadButton>, Changed<Interaction>)>,
    buildings: Query<&BuildingSlot>,
    roads: Query<&RoadSlot>,
    board: Res<Board>,
    players: Res<Players>,
    mut turn: ResMut<Turn>,
    mut hands: ResMut<Hands>,
) {
    if let Some((player, setup)) = match *turn {
        // We're in a road-building phase of a setup round
        // Show the buttons if we've just entered the phase
        Turn::Setup {
            player, road: true, ..
        } => turn.is_changed().then(|| (player, true)),
        // We're in a build phase
        // Show the buttons if the player pressed the button to build, and has enough resources
        Turn::Build { player } => build_buttons.get_single().ok().and_then(|interaction| {
            if let Interaction::Clicked = interaction {
                let hand = hands[players[player] as usize];

                (hand[Resource::Brick as usize] >= 1 && hand[Resource::Lumber as usize] >= 1)
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
            let visible = if setup {
                // We're in a setup phase
                // The player must build a road here if it's next to the building of their color that isn't next to any roads
                ROAD_BUILDING_ADJACENCY[**index]
                    .into_iter()
                    .any(|building| {
                        buildings
                            .get(board.buildings[building])
                            .unwrap()
                            .map(|building| building.color == color)
                            .unwrap_or(false)
                            && BUILDING_ROAD_ADJACENCY[building]
                                .iter()
                                .all(|road| roads.get(board.roads[*road]).unwrap().is_none())
                    })
            } else {
                // We're in a build phase
                // The player may build a road here if it's adjacent to another road of the same color
                roads.get(board.roads[**index]).unwrap().is_none()
                    && ROAD_ROAD_ADJACENCY[**index].iter().any(|road| {
                        roads
                            .get(board.roads[*road])
                            .unwrap()
                            .map_or(false, |road| color == road.color)
                    })
            };

            visibility.is_visible = visible;
            can_build |= visible;
        }

        // If they aren't in setup, they should be charged for the build
        if can_build && !setup {
            let hand = &mut hands[color as usize];

            hand[Resource::Brick as usize] -= 1;
            hand[Resource::Lumber as usize] -= 1;

            *turn = Turn::BuildRoad { player };
        }
    }
}

/// Build a road
fn build_road(
    mut commands: Commands,
    mut clicked_buttons: Query<(Entity, &BoardIndex), (With<RoadButton>, With<Clicked>)>,
    mut buttons: Query<&mut Visibility, With<RoadButton>>,
    mut roads: Query<&mut RoadSlot>,
    board: Res<Board>,
    players: Res<Players>,
    mut turn: ResMut<Turn>,
) {
    // We're in a road-building phase of a setup round
    if let Turn::Setup {
        player, road: true, ..
    }
    // We're building a road because the player pressed the Build road button
    | Turn::BuildRoad { player } = *turn
    {
        for (entity, index) in clicked_buttons.iter_mut() {
            commands.entity(entity).remove::<Clicked>();

            **roads.get_mut(board.roads[**index]).unwrap() = Some(Road {
                color: players[player],
            });

            for mut visibility in buttons.iter_mut() {
                visibility.is_visible = false;
            }

            *turn = turn.next(*players);
        }
    }
}
