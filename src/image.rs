use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

use crate::{
    building::{show_building_buttons, BuildingSlot},
    button::BuildingButton,
    chit::ChitSlot,
    harbor::HarborSlot,
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
            .add_system(BuildingSlot::update_images)
            .add_system(add_button_image.after(show_building_buttons));
    }
}

pub trait UpdateImages: Component + Copy + Sized {
    fn image(self) -> Option<&'static str>;
    fn update_images(
        mut commands: Commands,
        query: Query<(Entity, &Self, &Transform), Changed<Self>>,
        assets: Res<AssetServer>,
    ) {
        for (entity, component, transform) in query.iter() {
            let image = component.image();

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

static BUILDING_BUTTON_IMAGE: &str = "building_button.png";

fn add_button_image(
    mut commands: Commands,
    building_buttons: Query<(Entity, &Transform, &Visibility), Added<BuildingButton>>,
    assets: Res<AssetServer>,
) {
    for (entity, transform, visibility) in building_buttons.iter() {
        commands.entity(entity).insert_bundle(SpriteBundle {
            transform: *transform,
            texture: assets.load(BUILDING_BUTTON_IMAGE),
            visibility: visibility.clone(),
            ..default()
        });
    }
}
