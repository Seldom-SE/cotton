use bevy::{prelude::*, ui::FocusPolicy};

use crate::{color::PlayerColor, turn::Players};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_ui);
    }
}

/// Marks an entity as displaying what resources a player has, via children
#[derive(Component)]
pub struct HandUi {
    pub color: PlayerColor,
}

#[derive(Component)]
pub struct NextButton;

#[derive(Component)]
pub struct BuildRoadButton;

#[derive(Component)]
pub struct BuildSettlementButton;

#[derive(Component)]
pub struct Die1;

#[derive(Component)]
pub struct Die2;

#[derive(Component)]
pub struct StatusBar;

const PLAYER_HEADING_SIZE: f32 = 50.;
const BUTTON_FONT_SIZE: f32 = 30.;
const STATUS_FONT_SIZE: f32 = 40.;
const UI_BACKGROUND_COLOR: Color = Color::rgb(0.024, 0., 0.275);

/// Setup the game's UI
fn init_ui(mut commands: Commands, players: Res<Players>, assets: Res<AssetServer>) {
    // Root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // Left sidebar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(20.), Val::Percent(100.)),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    color: UI_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for player in players.into_iter() {
                        // Player heading
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                player,
                                TextStyle {
                                    font: assets.load("FiraSans-Bold.ttf"),
                                    font_size: PLAYER_HEADING_SIZE,
                                    color: player.into(),
                                },
                                default(),
                            ),
                            ..default()
                        });

                        // Player hand UI
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

            // Status bar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(60.), Val::Percent(10.)),
                        ..default()
                    },
                    color: UI_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Die 1
                    parent.spawn_bundle(ImageBundle::default()).insert(Die1);

                    // Die 2
                    parent.spawn_bundle(ImageBundle::default()).insert(Die2);

                    // Status text
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "",
                                TextStyle {
                                    font: assets.load("FiraSans-Bold.ttf"),
                                    font_size: STATUS_FONT_SIZE,
                                    color: default(),
                                },
                                default(),
                            ),
                            ..default()
                        })
                        .insert(StatusBar);
                });

            // Right sidebar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        size: Size::new(Val::Percent(20.), Val::Percent(100.)),
                        ..default()
                    },
                    color: UI_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Next button
                    parent
                        .spawn_bundle(ButtonBundle::default())
                        .insert(NextButton)
                        .with_children(|parent| {
                            // Button text
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Next",
                                    TextStyle {
                                        font: assets.load("FiraSans-Bold.ttf"),
                                        font_size: BUTTON_FONT_SIZE,
                                        color: Color::BLACK,
                                    },
                                    default(),
                                ),
                                focus_policy: FocusPolicy::Pass,
                                ..default()
                            });
                        });

                    // Build settlement button
                    parent
                        .spawn_bundle(ButtonBundle::default())
                        .insert(BuildSettlementButton)
                        .with_children(|parent| {
                            // Button text
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Build settlement",
                                    TextStyle {
                                        font: assets.load("FiraSans-Bold.ttf"),
                                        font_size: BUTTON_FONT_SIZE,
                                        color: Color::BLACK,
                                    },
                                    default(),
                                ),
                                focus_policy: FocusPolicy::Pass,
                                ..default()
                            });
                        });

                    // Build road button
                    parent
                        .spawn_bundle(ButtonBundle::default())
                        .insert(BuildRoadButton)
                        .with_children(|parent| {
                            // Button text
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Build road",
                                    TextStyle {
                                        font: assets.load("FiraSans-Bold.ttf"),
                                        font_size: BUTTON_FONT_SIZE,
                                        color: Color::BLACK,
                                    },
                                    default(),
                                ),
                                focus_policy: FocusPolicy::Pass,
                                ..default()
                            });
                        });
                });
        });
}
