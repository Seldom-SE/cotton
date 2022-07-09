use bevy::prelude::*;

use crate::{
    board::{Board, BoardIndex, BUILDING_BUILDING_ADJACENCY, BUILDING_TILE_ADJACENCY},
    button::{BuildingButton, Clicked},
    color::PlayerColor,
    image::UpdateImages,
    resource::Hands,
    tile::Tile,
    turn::{Players, Turn},
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

pub fn show_building_buttons(
    mut buttons: Query<(&mut Visibility, &BoardIndex), With<BuildingButton>>,
    buildings: Query<&BuildingSlot>,
    board: Res<Board>,
    turn: Res<Turn>,
) {
    if turn.is_changed() {
        if let Turn::Setup { road: false, .. } = *turn {
            for (mut visibility, index) in buttons.iter_mut() {
                visibility.is_visible = buildings.get(board.buildings[**index]).unwrap().is_none()
                    && BUILDING_BUILDING_ADJACENCY[**index].iter().all(|building| {
                        buildings.get(board.buildings[*building]).unwrap().is_none()
                    });
            }
        }
    }
}

fn build_settlement(
    mut commands: Commands,
    mut clicked_buttons: Query<(Entity, &BoardIndex), (With<BuildingButton>, With<Clicked>)>,
    mut buttons: Query<&mut Visibility, With<BuildingButton>>,
    mut buildings: Query<&mut BuildingSlot>,
    tiles: Query<&Tile>,
    board: Res<Board>,
    players: Res<Players>,
    mut turn: ResMut<Turn>,
    mut hands: ResMut<Hands>,
) {
    if let Turn::Setup {
        round_2,
        player,
        road: false,
    } = *turn
    {
        for (entity, index) in clicked_buttons.iter_mut() {
            commands.entity(entity).remove::<Clicked>();

            **buildings.get_mut(board.buildings[**index]).unwrap() = Some(Building {
                building_type: BuildingType::Settlement,
                color: players[player],
            });

            if round_2 {
                let hand = &mut hands[players[player] as usize];
                for tile in BUILDING_TILE_ADJACENCY[**index] {
                    if let Some(resource) = tiles.get(board.tiles[*tile]).unwrap().resource() {
                        hand[resource as usize] += 1;
                    }
                }
            }

            for mut visibility in buttons.iter_mut() {
                visibility.is_visible = false;
            }

            *turn = turn.next();
        }
    }
}
