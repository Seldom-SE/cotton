use bevy::prelude::*;

use crate::{button::BuildingButton, color::Color, turn::Turn};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_building_buttons);
    }
}

enum BuildingType {
    Settlement,
    City,
}

pub struct Building {
    building_type: BuildingType,
    color: Color,
}

#[derive(Component)]
pub struct BuildingSlot(pub Option<Building>);

pub fn show_building_buttons(
    mut buttons: Query<&mut Visibility, With<BuildingButton>>,
    turn: Res<Turn>,
) {
    if turn.is_changed() {
        if let Turn::Setup { road: false, .. } = *turn {
            for mut visibility in buttons.iter_mut() {
                visibility.is_visible = true;
            }
        }
    }
}
