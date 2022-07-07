use bevy::prelude::*;

use crate::{asset::AssetMap, random::Shuffle};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_tile_images);
    }
}

#[derive(Clone, Component, Copy, Debug)]
pub enum Tile {
    Hills,
    Pasture,
    Mountains,
    Fields,
    Forest,
    Desert,
}

static TILES: &[Tile] = &[
    Tile::Hills,
    Tile::Pasture,
    Tile::Mountains,
    Tile::Fields,
    Tile::Forest,
    Tile::Desert,
];

impl Shuffle for Tile {
    fn pool() -> &'static [Self] {
        TILES
    }

    fn weight(self) -> f32 {
        match self {
            Self::Hills | Self::Mountains => 3.,
            Self::Pasture | Self::Fields | Self::Forest => 4.,
            Self::Desert => 1.,
        }
    }
}

impl Tile {
    pub fn robber_home(self) -> bool {
        match self {
            Self::Hills | Self::Pasture | Self::Mountains | Self::Fields | Self::Forest => false,
            Self::Desert => true,
        }
    }

    fn image(self) -> &'static str {
        match self {
            Self::Hills => "hills",
            Self::Pasture => "pasture",
            Self::Mountains => "mountains",
            Self::Fields => "fields",
            Self::Forest => "forest",
            Self::Desert => "desert",
        }
    }
}

fn update_tile_images(
    mut commands: Commands,
    tiles: Query<(Entity, &Tile, &Transform), Added<Tile>>,
    assets: Res<AssetServer>,
    mut images: ResMut<AssetMap<Image>>,
) {
    for (entity, tile, transform) in tiles.iter() {
        commands.entity(entity).insert_bundle(SpriteBundle {
            transform: *transform,
            texture: images.get(&tile.image().to_string(), &assets),
            ..default()
        });
    }
}
