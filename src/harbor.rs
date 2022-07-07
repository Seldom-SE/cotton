use bevy::prelude::*;

use crate::{random::Shuffle, resource::Resource};

#[derive(Clone, Copy)]
pub enum Harbor {
    Resource(Resource),
    Any,
}

// Array of every `Option<Harbor>` type
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

#[derive(Clone, Component, Copy)]
pub struct HarborSlot(pub Option<Harbor>);
