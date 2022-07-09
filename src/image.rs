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

pub trait UpdateImages: Component + Copy + Sized {
    fn image(self, index: usize) -> Option<&'static str>;

    fn update_images(
        mut commands: Commands,
        query: Query<(Entity, &Self, &BoardIndex, &Transform), Changed<Self>>,
        assets: Res<AssetServer>,
    ) {
        for (entity, component, index, transform) in query.iter() {
            let image = component.image(**index);

            commands.entity(entity).insert_bundle(SpriteBundle {
                transform: *transform,
                texture: if let Some(image) = image {
                    assets.load(image)
                } else {
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

pub trait ButtonImage: Component + Sized {
    fn image() -> &'static str;

    fn add_image(
        mut commands: Commands,
        building_buttons: Query<(Entity, &Transform, &Visibility), Added<Self>>,
        assets: Res<AssetServer>,
    ) {
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
