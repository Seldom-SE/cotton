use bevy::prelude::*;

use crate::{image::UpdateImages, random::Shuffle, resource::Resource};

#[derive(Clone, Copy)]
pub enum Harbor {
    Resource(Resource),
    Any,
}

/// Every `Option<Harbor>` variant
static HARBORS: &[Option<Harbor>] = &[
    None,
    Some(Harbor::Resource(Resource::Brick)),
    Some(Harbor::Resource(Resource::Wool)),
    Some(Harbor::Resource(Resource::Ore)),
    Some(Harbor::Resource(Resource::Grain)),
    Some(Harbor::Resource(Resource::Lumber)),
    Some(Harbor::Any),
];

impl Shuffle for Option<Harbor> {
    fn pool() -> &'static [Self] {
        HARBORS
    }

    fn weight(self) -> f32 {
        match self {
            Some(Harbor::Resource(_)) => 1.,
            Some(Harbor::Any) => 4.,
            None => 21.,
        }
    }
}

#[derive(Clone, Component, Copy, Deref)]
pub struct HarborSlot(pub Option<Harbor>);

impl UpdateImages for HarborSlot {
    fn image(self, _: usize) -> Option<&'static str> {
        match *self {
            None => None,
            Some(Harbor::Resource(Resource::Brick)) => Some("brick_harbor.png"),
            Some(Harbor::Resource(Resource::Wool)) => Some("wool_harbor.png"),
            Some(Harbor::Resource(Resource::Ore)) => Some("ore_harbor.png"),
            Some(Harbor::Resource(Resource::Grain)) => Some("grain_harbor.png"),
            Some(Harbor::Resource(Resource::Lumber)) => Some("lumber_harbor.png"),
            Some(Harbor::Any) => Some("any_harbor.png"),
        }
    }
}
