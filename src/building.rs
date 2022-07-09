use bevy::prelude::*;

use crate::{
    board::{Board, BoardIndex, BUILDING_BUILDING_ADJACENCY},
    button::{BuildingButton, Clicked},
    color::Color,
    image::UpdateImages,
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
    pub color: Color,
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
                Color::Blue => "blue_settlement.png",
                Color::Orange => "orange_settlement.png",
                Color::Red => "red_settlement.png",
                Color::White => "white_settlement.png",
            }),
            Some(Building {
                building_type: BuildingType::City,
                color,
            }) => Some(match color {
                Color::Blue => "blue_city.png",
                Color::Orange => "orange_city.png",
                Color::Red => "red_city.png",
                Color::White => "white_city.png",
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
    board: Res<Board>,
    players: Res<Players>,
    mut turn: ResMut<Turn>,
) {
    if let Turn::Setup {
        player,
        road: false,
        ..
    } = *turn
    {
        for (entity, index) in clicked_buttons.iter_mut() {
            commands.entity(entity).remove::<Clicked>();

            **buildings.get_mut(board.buildings[**index]).unwrap() = Some(Building {
                building_type: BuildingType::Settlement,
                color: players[player],
            });

            for mut visibility in buttons.iter_mut() {
                visibility.is_visible = false;
            }

            *turn = turn.next();
        }
    }
}
