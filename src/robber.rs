use bevy::prelude::*;

use crate::image::UpdateImages;

#[derive(Clone, Component, Copy, Deref)]
pub struct RobberSlot(pub bool);

impl UpdateImages for RobberSlot {
    fn image(self) -> Option<&'static str> {
        self.then(|| "robber.png")
    }
}
