#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use board::BoardPlugin;
use building::BuildingPlugin;
use button::ButtonPlugin;
use camera::CameraPlugin;
use cursor::CursorPlugin;
use image::ImagePlugin;
use resource::ResourcePlugin;
use road::RoadPlugin;
use status::StatusPlugin;
use turn::TurnPlugin;
use ui::UiPlugin;

// The greatest project structure known to man /s
mod array;
mod board;
mod building;
mod button;
mod camera;
mod chit;
mod color;
mod cursor;
mod development_card;
mod harbor;
mod image;
mod math;
mod random;
mod resource;
mod road;
mod robber;
mod status;
mod tile;
mod turn;
mod ui;

// Completely original title
static TITLE: &str = "Cotton";
const CLEAR_COLOR: Color = Color::rgb(0.114, 0.281, 0.846);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: TITLE.to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BoardPlugin)
        .add_plugin(BuildingPlugin)
        .add_plugin(ButtonPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(CursorPlugin)
        .add_plugin(ImagePlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(RoadPlugin)
        .add_plugin(StatusPlugin)
        .add_plugin(TurnPlugin)
        .add_plugin(UiPlugin)
        .insert_resource(ClearColor(CLEAR_COLOR))
        .run();
}
