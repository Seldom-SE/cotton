use bevy::prelude::*;
use board::BoardPlugin;
use camera::CameraPlugin;
use image::ImagePlugin;
use tile::TilePlugin;

mod array;
mod asset;
mod board;
mod building;
mod camera;
mod chit;
mod color;
mod development_card;
mod harbor;
mod image;
mod random;
mod resource;
mod road;
mod robber;
mod tile;

static TITLE: &str = "Cotton";
const CLEAR_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: TITLE.to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BoardPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ImagePlugin)
        .add_plugin(TilePlugin)
        .insert_resource(ClearColor(CLEAR_COLOR))
        .run();
}
