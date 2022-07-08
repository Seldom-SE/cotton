use bevy::prelude::*;

use crate::{cursor::CursorPosition, math::RectContains};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(press_button);
    }
}

#[derive(Component)]
pub struct BoardButton {
    pub index: usize,
}

#[derive(Component)]
pub struct BuildingButton;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Clicked;

enum ButtonType {
    Building,
}

const BUILDING_BUTTON_RADIUS: f32 = 16.;

impl ButtonType {
    fn radius(self) -> f32 {
        match self {
            ButtonType::Building => BUILDING_BUTTON_RADIUS,
        }
    }
}

fn press_button(
    mut commands: Commands,
    buttons: Query<(Entity, Option<&BuildingButton>, &Transform, &Visibility), With<BoardButton>>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<Input<MouseButton>>,
) {
    if let Some(cursor_position) = **cursor_position {
        if mouse.just_pressed(MouseButton::Left) {
            for (entity, building_button, transform, visibility) in buttons.iter() {
                if visibility.is_visible {
                    let radius = building_button
                        .map(|_| ButtonType::Building)
                        .unwrap()
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
