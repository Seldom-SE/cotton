use bevy::prelude::*;

/// Adds a function that checks whether I contain a point
pub trait RectContains {
    fn contains(self, point: Vec2) -> bool;
}

impl RectContains for Rect<f32> {
    fn contains(self, point: Vec2) -> bool {
        point.x < self.right && point.x > self.left && point.y < self.top && point.y > self.bottom
    }
}
