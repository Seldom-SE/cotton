use bevy::prelude::*;

use crate::color::Color;

#[derive(Component)]
pub struct RoadSlot(pub Option<Road>);

pub struct Road {
    color: Color,
}
