use std::path::Path;

use bevy::{asset::Asset, prelude::*, utils::HashMap};

pub struct AssetMap<T: Asset> {
    assets: HashMap<String, Handle<T>>,
    path: String,
    extension: String,
}

static IMAGE_PATH: &str = "images";
static IMAGE_EXTENSION: &str = "png";

impl Default for AssetMap<Image> {
    fn default() -> Self {
        Self {
            assets: default(),
            path: IMAGE_PATH.to_string(),
            extension: IMAGE_EXTENSION.to_string(),
        }
    }
}

impl<T: Asset> AssetMap<T> {
    pub fn get(&mut self, name: &String, assets: &AssetServer) -> Handle<T> {
        self.assets.get(name).cloned().unwrap_or_else(|| {
            let handle = assets.load(
                Path::new(&self.path).join(Path::new(&format!("{}.{}", name, self.extension))),
            );
            self.assets.insert(name.clone(), handle.clone());
            handle
        })
    }
}
