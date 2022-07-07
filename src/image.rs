use bevy::prelude::*;

use crate::asset::AssetMap;

pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetMap<Image>>();
    }
}
