use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

use crate::{asset::AssetMap, random::Shuffle, resource::Resource};

pub struct HarborPlugin;

impl Plugin for HarborPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_harbor_images);
    }
}

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

impl Harbor {
    fn image(self) -> &'static str {
        match self {
            Self::Resource(Resource::Brick) => "brick_harbor",
            Self::Resource(Resource::Wool) => "wool_harbor",
            Self::Resource(Resource::Ore) => "ore_harbor",
            Self::Resource(Resource::Grain) => "grain_harbor",
            Self::Resource(Resource::Lumber) => "lumber_harbor",
            Self::Any => "any_harbor",
        }
    }
}

#[derive(Clone, Component, Copy, Deref)]
pub struct HarborSlot(pub Option<Harbor>);

fn update_harbor_images(
    mut commands: Commands,
    harbors: Query<(Entity, &HarborSlot, &Transform), Added<HarborSlot>>,
    assets: Res<AssetServer>,
    mut images: ResMut<AssetMap<Image>>,
) {
    for (entity, harbor, transform) in harbors.iter() {
        commands.entity(entity).insert_bundle(SpriteBundle {
            transform: *transform,
            texture: if let Some(harbor) = &**harbor {
                images.get(&harbor.image().to_string(), &assets)
            } else {
                DEFAULT_IMAGE_HANDLE.typed()
            },
            visibility: Visibility {
                is_visible: harbor.is_some(),
            },
            ..default()
        });
    }
}
