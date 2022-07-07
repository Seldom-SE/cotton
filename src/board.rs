use bevy::prelude::*;

use crate::{
    array::{enumerate, zip},
    building::BuildingSlot,
    chit::{Chit, ChitSlot},
    harbor::{Harbor, HarborSlot},
    random::Shuffle,
    road::RoadSlot,
    robber::RobberSlot,
    tile::Tile,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_board);
    }
}

const TILE_COUNT: usize = 19;
const HARBOR_COUNT: usize = 30;
const ROAD_COUNT: usize = 72;
const BUILDING_COUNT: usize = 56;

struct Board {
    tiles: [Entity; TILE_COUNT],
    chits: [Entity; TILE_COUNT],
    robbers: [Entity; TILE_COUNT],
    harbors: [Entity; HARBOR_COUNT],
    roads: [Entity; ROAD_COUNT],
    buildings: [Entity; BUILDING_COUNT],
}

const TILE_POSITIONS: [(f32, f32); TILE_COUNT] = [
    (-110., 190.),
    (0., 190.),
    (110., 190.),
    (-165., 95.),
    (-55., 95.),
    (55., 95.),
    (165., 95.),
    (-220., 0.),
    (-110., 0.),
    (0., 0.),
    (110., 0.),
    (220., 0.),
    (-165., -95.),
    (-55., -95.),
    (55., -95.),
    (165., -95.),
    (-110., -190.),
    (0., -190.),
    (110., -190.),
];

const TILE_Z: f32 = 0.;
const CHIT_Z: f32 = 1.;
const ROBBER_Z: f32 = 1.;
const HARBOR_Z: f32 = 0.;
const ROAD_Z: f32 = 1.;
const BUILDING_Z: f32 = 1.;

fn generate_board(mut commands: Commands) {
    let tiles = Tile::shuffle();

    let board = Board {
        tiles: enumerate(tiles).map(|(i, tile)| {
            commands
                .spawn()
                .insert(tile)
                .insert(Transform::from_translation(
                    Vec2::from(TILE_POSITIONS[i]).extend(TILE_Z),
                ))
                .id()
        }),
        chits: enumerate(zip(Chit::shuffle(), tiles)).map(|(i, (chit, tile))| {
            commands
                .spawn()
                .insert(ChitSlot((!tile.robber_home()).then(|| chit)))
                .insert(Transform::from_translation(
                    Vec2::from(TILE_POSITIONS[i]).extend(CHIT_Z),
                ))
                .id()
        }),
        robbers: enumerate(tiles).map(|(i, tile)| {
            commands
                .spawn()
                .insert(RobberSlot(tile.robber_home()))
                .insert(Transform::from_translation(
                    Vec2::from(TILE_POSITIONS[i]).extend(ROBBER_Z),
                ))
                .id()
        }),
        harbors: Option::<Harbor>::shuffle()
            .map(|harbor| commands.spawn().insert(HarborSlot(harbor)).id()),
        roads: [(); ROAD_COUNT].map(|_| commands.spawn().insert(RoadSlot(None)).id()),
        buildings: [(); BUILDING_COUNT].map(|_| commands.spawn().insert(BuildingSlot(None)).id()),
    };

    commands.insert_resource(board);
}
