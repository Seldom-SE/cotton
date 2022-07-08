use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

use crate::{chit::ChitSlot, harbor::HarborSlot, tile::Tile};

pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Tile::update_images)
            .add_system(ChitSlot::update_images)
            .add_system(HarborSlot::update_images);
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
