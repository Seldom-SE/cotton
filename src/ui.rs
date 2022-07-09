use bevy::prelude::*;

use crate::{color::PlayerColor, turn::Players};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_ui);
    }
}

const FONT_SIZE: f32 = 50.;

#[derive(Component)]
pub struct HandUi {
    pub color: PlayerColor,
}

fn init_ui(mut commands: Commands, players: Res<Players>, assets: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(20.), Val::Percent(100.)),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    color: Color::rgb(0.024, 0., 0.275).into(),
                    ..default()
                })
                .with_children(|parent| {
                    for player in players.into_iter() {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                player,
                                TextStyle {
                                    font: assets.load("FiraSans-Bold.ttf"),
                                    font_size: FONT_SIZE,
                                    color: player.into(),
                                },
                                default(),
                            ),
                            ..default()
                        });

                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Auto),
                                    ..default()
                                },
                                color: Color::NONE.into(),
                                ..default()
                            })
                            .insert(HandUi { color: player });
                    }
                });
        });
}
