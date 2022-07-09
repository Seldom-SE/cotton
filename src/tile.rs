use bevy::prelude::*;

use crate::{image::UpdateImages, random::Shuffle};

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
}

impl UpdateImages for Tile {
    fn image(self, _: usize) -> Option<&'static str> {
        match self {
            Self::Hills => Some("hills.png"),
            Self::Pasture => Some("pasture.png"),
            Self::Mountains => Some("mountains.png"),
            Self::Fields => Some("fields.png"),
            Self::Forest => Some("forest.png"),
            Self::Desert => Some("desert.png"),
        }
    }
}
