use bevy::prelude::*;

use crate::{cursor::CursorPosition, image::ButtonImage, math::RectContains};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(press_button);
    }
}

/// Buttons that appear on the nodes of the board, for building settlements and cities
#[derive(Component)]
pub struct BuildingButton;

impl ButtonImage for BuildingButton {
    fn image() -> &'static str {
        "building_button.png"
    }
}

/// Buttons that appear on the edges of the board, for building roads
#[derive(Component)]
pub struct RoadButton;

impl ButtonImage for RoadButton {
    fn image() -> &'static str {
        "building_button.png"
    }
}

/// Used on non-UI buttons
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Clicked;

pub enum ButtonType {
    Building,
    Road,
}

// These are radii for, uh, squares
const BUILDING_BUTTON_RADIUS: f32 = 16.;
const ROAD_BUTTON_RADIUS: f32 = 16.;

impl ButtonType {
    fn radius(self) -> f32 {
        match self {
            ButtonType::Building => BUILDING_BUTTON_RADIUS,
            ButtonType::Road => ROAD_BUTTON_RADIUS,
        }
    }
}

/// Add `Clicked` component to buttons that were just clicked
fn press_button(
    mut commands: Commands,
    buttons: Query<
        (
            Entity,
            Option<&BuildingButton>,
            Option<&RoadButton>,
            &Transform,
            &Visibility,
        ),
        Or<(With<BuildingButton>, With<RoadButton>)>,
    >,
    cursor_position: Res<CursorPosition>,
    mouse: Res<Input<MouseButton>>,
) {
    if let Some(cursor_position) = **cursor_position {
        if mouse.just_pressed(MouseButton::Left) {
            for (entity, building_button, road_button, transform, visibility) in buttons.iter() {
                if visibility.is_visible {
                    // Figure out what button type we're using, and get the radius
                    let radius = building_button
                        .map(|_| ButtonType::Building)
                        .unwrap_or_else(|| road_button.map(|_| ButtonType::Road).unwrap())
                        .radius();
                    let translation = transform.translation;

                    if (Rect {
                        left: translation.x - radius,
                        right: translation.x + radius,
                        top: translation.y + radius,
                        bottom: translation.y - radius,
                    }
                    .contains(cursor_position))
                    {
                        commands.entity(entity).insert(Clicked);
                    }
                }
            }
        }
    }
}
