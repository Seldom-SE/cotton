use bevy::prelude::*;

use crate::color::Color;

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
