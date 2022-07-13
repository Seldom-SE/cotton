use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    board::{Board, BoardIndex, BUILDING_TILE_ADJACENCY},
    building::BuildingSlot,
    chit::ChitSlot,
    tile::Tile,
    turn::{Turn, PLAYER_COUNT},
    ui::{Die1, Die2, HandUi},
};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hands>()
            .add_system(update_hand_ui)
            .add_system(produce_resources);
    }
}

#[derive(Clone, Copy)]
pub enum Resource {
    Brick,
    Wool,
    Ore,
    Grain,
    Lumber,
}

impl Resource {
    fn image(self) -> &'static str {
        match self {
            Self::Brick => "brick.png",
            Self::Wool => "wool.png",
            Self::Ore => "ore.png",
            Self::Grain => "grain.png",
            Self::Lumber => "lumber.png",
        }
    }
}

const RESOURCE_COUNT: usize = 5;

/// Indexed by a `PlayerColor as usize`, and then a `Resource as usize`
#[derive(Default, Deref, DerefMut)]
pub struct Hands([[u8; RESOURCE_COUNT]; PLAYER_COUNT]);

const RESOURCES: [Resource; RESOURCE_COUNT] = [
    Resource::Brick,
    Resource::Wool,
    Resource::Ore,
    Resource::Grain,
    Resource::Lumber,
];
const RESOURCE_SIZE: Val = Val::Px(32.);

/// When a player's hand changes, update the hand in the UI
fn update_hand_ui(
    mut commands: Commands,
    hand_uis: Query<(Entity, &HandUi)>,
    hands: Res<Hands>,
    assets: Res<AssetServer>,
) {
    if hands.is_changed() {
        for (entity, hand) in hand_uis.iter() {
            let mut hand_commands = commands.entity(entity);
            hand_commands.despawn_descendants();

            hand_commands.with_children(|parent| {
                for (resource, count) in hands[hand.color as usize].into_iter().enumerate() {
                    for _ in 0..count {
                        parent.spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(RESOURCE_SIZE, RESOURCE_SIZE),
                                ..default()
                            },
                            image: assets.load(RESOURCES[resource].image()).into(),
                            ..default()
                        });
                    }
                }
            });
        }
    }
}

/// Roll the dice and give the players their resources
fn produce_resources(
    buildings: Query<(&BuildingSlot, &BoardIndex)>,
    tiles: Query<&Tile>,
    chits: Query<&ChitSlot>,
    mut die_1s: Query<&mut UiImage, (With<Die1>, Without<Die2>)>,
    mut die_2s: Query<&mut UiImage, (With<Die2>, Without<Die1>)>,
    board: Res<Board>,
    assets: Res<AssetServer>,
    mut hands: ResMut<Hands>,
    mut turn: ResMut<Turn>,
) {
    if turn.is_changed() {
        if let Turn::Production { .. } = *turn {
            let mut rng = thread_rng();

            let roll_1 = rng.gen_range(1..=6);
            let roll_2 = rng.gen_range(1..=6);

            for (roll, mut image) in
                [(roll_1, die_1s.single_mut()), (roll_2, die_2s.single_mut())].into_iter()
            {
                *image = assets
                    .load(match roll {
                        1 => "die_1.png",
                        2 => "die_2.png",
                        3 => "die_3.png",
                        4 => "die_4.png",
                        5 => "die_5.png",
                        6 => "die_6.png",
                        _ => panic!("Invalid die roll"),
                    })
                    .into();
            }

            let total = roll_1 + roll_2;

            for (building, index) in buildings.iter() {
                if let Some(building) = **building {
                    let hand = &mut hands[building.color as usize];

                    for tile in BUILDING_TILE_ADJACENCY[**index] {
                        if let Some(chit) = **chits.get(board.chits[*tile]).unwrap() {
                            if total == *chit {
                                hand[tiles.get(board.tiles[*tile]).unwrap().resource().unwrap()
                                    as usize] += building.building_type.production();
                            }
                        }
                    }
                }
            }

            *turn = turn.next();
        }
    }
}
