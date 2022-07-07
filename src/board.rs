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

const HARBOR_POSITIONS: [(f32, f32); HARBOR_COUNT] = [
    (-151.25, 261.25),
    (-68.75, 261.25),
    (-41.25, 261.25),
    (41.25, 261.25),
    (68.75, 261.25),
    (151.25, 261.25),
    (192.5, 190.),
    (206.25, 166.25),
    (247.5, 95.),
    (261.25, 71.25),
    (302.5, 0.),
    (261.25, -71.25),
    (247.5, -95.),
    (206.25, -166.25),
    (192.5, -190.),
    (151.25, -261.25),
    (68.75, -261.25),
    (41.25, -261.25),
    (-41.25, -261.25),
    (-68.75, -261.25),
    (-151.25, -261.25),
    (-192.5, -190.),
    (-206.25, -166.25),
    (-247.5, -95.),
    (-261.25, -71.25),
    (-302.5, 0.),
    (-261.25, 71.25),
    (-247.5, 95.),
    (-206.25, 166.25),
    (-192.5, 190.),
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
        harbors: enumerate(Option::<Harbor>::shuffle()).map(|(i, harbor)| {
            commands
                .spawn()
                .insert(HarborSlot(harbor))
                .insert(Transform::from_translation(
                    Vec2::from(HARBOR_POSITIONS[i]).extend(HARBOR_Z),
                ))
                .id()
        }),
        roads: [(); ROAD_COUNT].map(|_| commands.spawn().insert(RoadSlot(None)).id()),
        buildings: [(); BUILDING_COUNT].map(|_| commands.spawn().insert(BuildingSlot(None)).id()),
    };

    commands.insert_resource(board);
}
