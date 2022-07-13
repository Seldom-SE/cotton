use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

use crate::{
    board::BoardIndex,
    building::{show_building_buttons, BuildingSlot},
    button::{BuildingButton, RoadButton},
    chit::ChitSlot,
    harbor::HarborSlot,
    road::RoadSlot,
    robber::RobberSlot,
    tile::Tile,
};

pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Tile::update_images)
            .add_system(ChitSlot::update_images)
            .add_system(RobberSlot::update_images)
            .add_system(HarborSlot::update_images)
            .add_system(RoadSlot::update_images)
            .add_system(BuildingSlot::update_images)
            .add_system(BuildingButton::add_image.after(show_building_buttons))
            .add_system(RoadButton::add_image);
    }
}

/// You implement the `image` function for me, and I'll implement an `update_images` system for you.
/// Should only be used on components that appear with `BoardIndex`.
/// Remember to add `update_images` to the app!
pub trait UpdateImages: Component + Copy + Sized {
    /// Path to your image asset, if you have one
    fn image(self, index: usize) -> Option<&'static str>;

    /// Remember to add me to the app!
    fn update_images(
        mut commands: Commands,
        query: Query<(Entity, &Self, &BoardIndex, &Transform), Changed<Self>>,
        assets: Res<AssetServer>,
    ) {
        for (entity, component, index, transform) in query.iter() {
            let image = component.image(**index);

            // The entity is allowed to not already have a `SpriteBundle`, so we use `Commands` to add/overwrite it
            commands.entity(entity).insert_bundle(SpriteBundle {
                transform: *transform,
                texture: if let Some(image) = image {
                    assets.load(image)
                } else {
                    // We really love `null` in Rust, don't we
                    DEFAULT_IMAGE_HANDLE.typed()
                },
                visibility: Visibility {
                    is_visible: image.is_some(),
                },
                ..default()
            });
        }
    }
}

/// You implement the `image` function for me, and I'll implement an `add_image` system for you.
/// Remember to add `app_image` to the app!
pub trait ButtonImage: Component + Sized {
    /// Path to your image asset
    fn image() -> &'static str;

    /// Remember to add me to the app!
    fn add_image(
        mut commands: Commands,
        building_buttons: Query<(Entity, &Transform, &Visibility), Added<Self>>,
        assets: Res<AssetServer>,
    ) {
        // A button's image never changes, so we only add the image on create
        for (entity, transform, visibility) in building_buttons.iter() {
            commands.entity(entity).insert_bundle(SpriteBundle {
                transform: *transform,
                texture: assets.load(Self::image()),
                visibility: visibility.clone(),
                ..default()
            });
        }
    }
}
