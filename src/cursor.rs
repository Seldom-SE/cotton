use bevy::{prelude::*, render::camera::Camera2d};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .add_system(get_cursor_position);
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct CursorPosition(Option<Vec2>);

fn get_cursor_position(
    cameras: Query<&Transform, With<Camera2d>>,
    windows: Res<Windows>,
    mut position: ResMut<CursorPosition>,
) {
    if let Ok(transform) = cameras.get_single() {
        let window = windows.get_primary().unwrap();
        **position = window.cursor_position().map(|cursor_position| {
            (transform.compute_matrix()
                * (cursor_position - Vec2::new(window.width(), window.height()) / 2.)
                    .extend(0.)
                    .extend(1.))
            .truncate()
            .truncate()
        })
    }
}
