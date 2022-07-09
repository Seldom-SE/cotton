use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    board::{Board, BoardIndex, BUILDING_TILE_ADJACENCY},
    building::BuildingSlot,
    chit::ChitSlot,
    tile::Tile,
    turn::{Players, Turn, PLAYER_COUNT},
    ui::HandUi,
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

fn produce_resources(
    buildings: Query<(&BuildingSlot, &BoardIndex)>,
    tiles: Query<&Tile>,
    chits: Query<&ChitSlot>,
    board: Res<Board>,
    players: Res<Players>,
    mut hands: ResMut<Hands>,
    mut turn: ResMut<Turn>,
) {
    if turn.is_changed() {
        if let Turn::Production { .. } = *turn {
            let mut rng = thread_rng();
            let roll = rng.gen_range(1..=6) + rng.gen_range(1..=6);

            println!("Rolled a {roll}");

            for (building, index) in buildings.iter() {
                if let Some(building) = **building {
                    let hand = &mut hands[building.color as usize];

                    for tile in BUILDING_TILE_ADJACENCY[**index] {
                        if let Some(chit) = **chits.get(board.chits[*tile]).unwrap() {
                            if roll == *chit {
                                hand[tiles.get(board.tiles[*tile]).unwrap().resource().unwrap()
                                    as usize] += building.building_type.production();
                            }
                        }
                    }
                }
            }

            *turn = turn.next(*players);
        }
    }
}
