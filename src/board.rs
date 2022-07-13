use bevy::prelude::*;

use crate::{
    array::{enumerate, zip},
    building::BuildingSlot,
    button::{BuildingButton, RoadButton},
    chit::{Chit, ChitSlot},
    harbor::{Harbor, HarborSlot},
    random::Shuffle,
    road::{RoadOrientation, RoadSlot},
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
const BUILDING_COUNT: usize = 54;

pub struct Board {
    pub tiles: [Entity; TILE_COUNT],
    pub chits: [Entity; TILE_COUNT],
    pub robbers: [Entity; TILE_COUNT],
    pub harbors: [Entity; HARBOR_COUNT],
    pub roads: [Entity; ROAD_COUNT],
    pub road_buttons: [Entity; ROAD_COUNT],
    pub buildings: [Entity; BUILDING_COUNT],
    pub building_buttons: [Entity; BUILDING_COUNT],
}

/// Added to board items (ex. `Tile`s, `RoadSlot`s).
/// Represents an index that identifies which item it is specifically,
#[derive(Component, Deref)]
pub struct BoardIndex(usize);

/// Maps a tile's board index to its position
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

/// Maps a harbor's board index to its position
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

/// Maps a road's board index to its position
const ROAD_POSITIONS: [(f32, f32); ROAD_COUNT] = [
    (-137.5, 237.5),
    (-82.5, 237.5),
    (-27.5, 237.5),
    (27.5, 237.5),
    (82.5, 237.5),
    (137.5, 237.5),
    (-165., 190.),
    (-55., 190.),
    (55., 190.),
    (165., 190.),
    (-192.5, 142.5),
    (-137.5, 142.5),
    (-82.5, 142.5),
    (-27.5, 142.5),
    (27.5, 142.5),
    (82.5, 142.5),
    (137.5, 142.5),
    (192.5, 142.5),
    (-220., 95.),
    (-110., 95.),
    (0., 95.),
    (110., 95.),
    (220., 95.),
    (-247.5, 47.5),
    (-192.5, 47.5),
    (-137.5, 47.5),
    (-82.5, 47.5),
    (-27.5, 47.5),
    (27.5, 47.5),
    (82.5, 47.5),
    (137.5, 47.5),
    (192.5, 47.5),
    (247.5, 47.5),
    (-275., 0.),
    (-165., 0.),
    (-55., 0.),
    (55., 0.),
    (165., 0.),
    (275., 0.),
    (-247.5, -47.5),
    (-192.5, -47.5),
    (-137.5, -47.5),
    (-82.5, -47.5),
    (-27.5, -47.5),
    (27.5, -47.5),
    (82.5, -47.5),
    (137.5, -47.5),
    (192.5, -47.5),
    (247.5, -47.5),
    (-220., -95.),
    (-110., -95.),
    (0., -95.),
    (110., -95.),
    (220., -95.),
    (-192.5, -142.5),
    (-137.5, -142.5),
    (-82.5, -142.5),
    (-27.5, -142.5),
    (27.5, -142.5),
    (82.5, -142.5),
    (137.5, -142.5),
    (192.5, -142.5),
    (-165., -190.),
    (-55., -190.),
    (55., -190.),
    (165., -190.),
    (-137.5, -237.5),
    (-82.5, -237.5),
    (-27.5, -237.5),
    (27.5, -237.5),
    (82.5, -237.5),
    (137.5, -237.5),
];

/// Maps a road's board index to its visual orientation
pub const ROAD_ORIENTATIONS: [RoadOrientation; ROAD_COUNT] = [
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Vert,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
    RoadOrientation::Dec,
    RoadOrientation::Inc,
];

/// Maps a road's board index to the board indices of adjacent roads
pub const ROAD_ROAD_ADJACENCY: [&[usize]; ROAD_COUNT] = [
    &[1, 6],
    &[0, 2, 7],
    &[1, 3, 7],
    &[2, 4, 8],
    &[3, 5, 8],
    &[4, 9],
    &[0, 10, 11],
    &[1, 2, 12, 13],
    &[3, 4, 14, 15],
    &[5, 16, 17],
    &[6, 11, 18],
    &[6, 10, 12, 19],
    &[7, 11, 13, 19],
    &[7, 12, 14, 20],
    &[8, 13, 15, 20],
    &[8, 14, 16, 21],
    &[9, 15, 17, 21],
    &[9, 16, 22],
    &[10, 23, 24],
    &[11, 12, 25, 26],
    &[13, 14, 27, 28],
    &[15, 16, 29, 30],
    &[17, 31, 32],
    &[18, 24, 33],
    &[18, 23, 25, 34],
    &[19, 24, 26, 34],
    &[19, 25, 27, 35],
    &[20, 26, 28, 35],
    &[20, 27, 29, 36],
    &[21, 28, 30, 36],
    &[21, 29, 31, 37],
    &[22, 30, 32, 37],
    &[22, 31, 38],
    &[23, 39],
    &[24, 25, 40, 41],
    &[26, 27, 42, 43],
    &[28, 29, 44, 45],
    &[30, 31, 46, 47],
    &[32, 48],
    &[33, 40, 49],
    &[34, 39, 41, 49],
    &[34, 40, 42, 50],
    &[35, 41, 43, 50],
    &[35, 42, 44, 51],
    &[36, 43, 45, 51],
    &[36, 44, 46, 52],
    &[37, 45, 47, 52],
    &[37, 46, 48, 53],
    &[38, 47, 53],
    &[39, 40, 54],
    &[41, 42, 55, 56],
    &[43, 44, 57, 58],
    &[45, 46, 59, 60],
    &[47, 48, 61],
    &[49, 55, 62],
    &[50, 54, 56, 62],
    &[50, 55, 57, 63],
    &[51, 56, 58, 63],
    &[51, 57, 59, 64],
    &[52, 58, 60, 64],
    &[52, 59, 61, 65],
    &[53, 60, 65],
    &[54, 55, 66],
    &[56, 57, 67, 68],
    &[58, 59, 69, 70],
    &[60, 61, 71],
    &[62, 67],
    &[63, 66, 68],
    &[63, 67, 69],
    &[64, 68, 70],
    &[64, 69, 71],
    &[65, 70],
];

/// Maps a road's board index to the board indices of adjacent buildings
pub const ROAD_BUILDING_ADJACENCY: [[usize; 2]; ROAD_COUNT] = [
    [0, 1],
    [1, 2],
    [2, 3],
    [3, 4],
    [4, 5],
    [5, 6],
    [0, 8],
    [2, 10],
    [4, 12],
    [6, 14],
    [7, 8],
    [8, 9],
    [9, 10],
    [10, 11],
    [11, 12],
    [12, 13],
    [13, 14],
    [14, 15],
    [7, 17],
    [9, 19],
    [11, 21],
    [13, 23],
    [15, 25],
    [16, 17],
    [17, 18],
    [18, 19],
    [19, 20],
    [20, 21],
    [21, 22],
    [22, 23],
    [23, 24],
    [24, 25],
    [25, 26],
    [16, 27],
    [18, 29],
    [20, 31],
    [22, 33],
    [24, 35],
    [26, 37],
    [27, 28],
    [28, 29],
    [29, 30],
    [30, 31],
    [31, 32],
    [32, 33],
    [33, 34],
    [34, 35],
    [35, 36],
    [36, 37],
    [28, 38],
    [30, 40],
    [32, 42],
    [34, 44],
    [36, 46],
    [38, 39],
    [39, 40],
    [40, 41],
    [41, 42],
    [42, 43],
    [43, 44],
    [44, 45],
    [45, 46],
    [39, 47],
    [41, 49],
    [43, 51],
    [45, 53],
    [47, 48],
    [48, 49],
    [49, 50],
    [50, 51],
    [51, 52],
    [52, 53],
];

/// Maps a building's board index to its position
const BUILDING_POSITIONS: [(f32, f32); BUILDING_COUNT] = [
    (-165., 222.),
    (-110., 253.),
    (-55., 222.),
    (0., 253.),
    (55., 222.),
    (110., 253.),
    (165., 222.),
    (-220., 127.),
    (-165., 158.),
    (-110., 127.),
    (-55., 158.),
    (0., 127.),
    (55., 158.),
    (110., 127.),
    (165., 158.),
    (220., 127.),
    (-275., 32.),
    (-220., 63.),
    (-165., 32.),
    (-110., 63.),
    (-55., 32.),
    (0., 63.),
    (55., 32.),
    (110., 63.),
    (165., 32.),
    (220., 63.),
    (275., 32.),
    (-275., -32.),
    (-220., -63.),
    (-165., -32.),
    (-110., -63.),
    (-55., -32.),
    (0., -63.),
    (55., -32.),
    (110., -63.),
    (165., -32.),
    (220., -63.),
    (275., -32.),
    (-220., -127.),
    (-165., -158.),
    (-110., -127.),
    (-55., -158.),
    (0., -127.),
    (55., -158.),
    (110., -127.),
    (165., -158.),
    (220., -127.),
    (-165., -222.),
    (-110., -253.),
    (-55., -222.),
    (0., -253.),
    (55., -222.),
    (110., -253.),
    (165., -222.),
];

/// Maps a building's board index to the board indices of adjacent tiles
pub const BUILDING_TILE_ADJACENCY: [&[usize]; BUILDING_COUNT] = [
    &[0],
    &[0],
    &[0, 1],
    &[1],
    &[1, 2],
    &[2],
    &[2],
    &[3],
    &[0, 3],
    &[0, 3, 4],
    &[0, 1, 4],
    &[1, 4, 5],
    &[1, 2, 5],
    &[2, 5, 6],
    &[2, 6],
    &[6],
    &[7],
    &[3, 7],
    &[3, 7, 8],
    &[3, 4, 8],
    &[4, 8, 9],
    &[4, 5, 9],
    &[5, 9, 10],
    &[5, 6, 10],
    &[6, 10, 11],
    &[6, 11],
    &[11],
    &[7],
    &[7, 12],
    &[7, 8, 12],
    &[8, 12, 13],
    &[8, 9, 13],
    &[9, 13, 14],
    &[9, 10, 14],
    &[10, 14, 15],
    &[10, 11, 15],
    &[11, 15],
    &[11],
    &[12],
    &[12, 16],
    &[12, 13, 16],
    &[13, 16, 17],
    &[13, 14, 17],
    &[14, 17, 18],
    &[14, 15, 18],
    &[15, 18],
    &[15],
    &[16],
    &[16],
    &[16, 17],
    &[17],
    &[17, 18],
    &[18],
    &[18],
];

/// Maps a building's board index to the board indices of adjacent roads
pub const BUILDING_ROAD_ADJACENCY: [&[usize]; BUILDING_COUNT] = [
    &[0, 6],
    &[0, 1],
    &[1, 2, 7],
    &[2, 3],
    &[3, 4, 8],
    &[4, 5],
    &[5, 9],
    &[10, 18],
    &[6, 10, 11],
    &[11, 12, 19],
    &[7, 12, 13],
    &[13, 14, 20],
    &[8, 14, 15],
    &[15, 16, 21],
    &[9, 16, 17],
    &[17, 22],
    &[23, 33],
    &[18, 23, 24],
    &[24, 25, 34],
    &[19, 25, 26],
    &[26, 27, 35],
    &[20, 27, 28],
    &[28, 29, 36],
    &[21, 29, 30],
    &[30, 31, 37],
    &[22, 31, 32],
    &[32, 38],
    &[33, 39],
    &[39, 40, 49],
    &[34, 40, 41],
    &[41, 42, 50],
    &[35, 42, 43],
    &[43, 44, 51],
    &[36, 44, 45],
    &[45, 46, 52],
    &[37, 46, 47],
    &[47, 48, 53],
    &[38, 48],
    &[49, 54],
    &[54, 55, 62],
    &[50, 55, 56],
    &[56, 57, 63],
    &[51, 57, 58],
    &[58, 59, 64],
    &[52, 59, 60],
    &[60, 61, 65],
    &[53, 61],
    &[62, 66],
    &[66, 67],
    &[63, 67, 68],
    &[68, 69],
    &[64, 69, 70],
    &[70, 71],
    &[65, 71],
];

/// Maps a building's board index to the board indices of adjacent buildings
pub const BUILDING_BUILDING_ADJACENCY: [&[usize]; BUILDING_COUNT] = [
    &[1, 8],
    &[0, 2],
    &[1, 3, 7],
    &[2, 4],
    &[3, 5, 12],
    &[4, 6],
    &[5, 14],
    &[8, 17],
    &[0, 7, 9],
    &[8, 10, 19],
    &[2, 9, 11],
    &[10, 12, 21],
    &[4, 11, 13],
    &[12, 14, 23],
    &[6, 13, 15],
    &[14, 25],
    &[17, 27],
    &[7, 16, 18],
    &[17, 19, 29],
    &[9, 18, 20],
    &[19, 21, 31],
    &[11, 20, 22],
    &[21, 23, 33],
    &[13, 22, 24],
    &[23, 25, 35],
    &[15, 24, 26],
    &[25, 37],
    &[16, 28],
    &[27, 29, 38],
    &[18, 28, 30],
    &[29, 31, 40],
    &[20, 30, 32],
    &[31, 33, 42],
    &[22, 32, 34],
    &[33, 35, 44],
    &[24, 34, 36],
    &[35, 37, 46],
    &[26, 36],
    &[28, 39],
    &[38, 40, 47],
    &[30, 39, 41],
    &[40, 42, 49],
    &[32, 41, 43],
    &[42, 44, 51],
    &[34, 43, 45],
    &[44, 46, 53],
    &[36, 45],
    &[39, 48],
    &[47, 49],
    &[41, 48, 50],
    &[49, 51],
    &[43, 50, 52],
    &[51, 53],
    &[45, 52],
];

const TILE_Z: f32 = 0.;
const BUTTON_Z: f32 = 3.;
const CHIT_Z: f32 = 1.;
const ROBBER_Z: f32 = 2.;
const HARBOR_Z: f32 = 0.;
const ROAD_Z: f32 = 1.;
const BUILDING_Z: f32 = 1.;

fn generate_board(mut commands: Commands) {
    let tiles = Tile::shuffle();

    // The `enumerate` calls here are used to generate board indices
    let board = Board {
        tiles: enumerate(tiles).map(|(i, tile)| {
            commands
                .spawn()
                .insert(tile)
                .insert(BoardIndex(i))
                .insert(Transform::from_translation(
                    Vec2::from(TILE_POSITIONS[i]).extend(TILE_Z),
                ))
                .id()
        }),
        chits: enumerate(zip(Chit::shuffle(), tiles)).map(|(i, (chit, tile))| {
            commands
                .spawn()
                .insert(ChitSlot((!tile.robber_home()).then(|| chit)))
                .insert(BoardIndex(i))
                .insert(Transform::from_translation(
                    Vec2::from(TILE_POSITIONS[i]).extend(CHIT_Z),
                ))
                .id()
        }),
        robbers: enumerate(tiles).map(|(i, tile)| {
            commands
                .spawn()
                .insert(RobberSlot(tile.robber_home()))
                .insert(BoardIndex(i))
                .insert(Transform::from_translation(
                    Vec2::from(TILE_POSITIONS[i]).extend(ROBBER_Z),
                ))
                .id()
        }),
        harbors: enumerate(Option::<Harbor>::shuffle()).map(|(i, harbor)| {
            commands
                .spawn()
                .insert(HarborSlot(harbor))
                .insert(BoardIndex(i))
                .insert(Transform::from_translation(
                    Vec2::from(HARBOR_POSITIONS[i]).extend(HARBOR_Z),
                ))
                .id()
        }),
        roads: enumerate([(); ROAD_COUNT]).map(|(i, _)| {
            commands
                .spawn()
                .insert(RoadSlot(None))
                .insert(BoardIndex(i))
                .insert(Transform::from_translation(
                    Vec2::from(ROAD_POSITIONS[i]).extend(ROAD_Z),
                ))
                .id()
        }),
        road_buttons: enumerate([(); ROAD_COUNT]).map(|(i, _)| {
            commands
                .spawn()
                .insert(RoadButton)
                .insert(BoardIndex(i))
                .insert(Transform::from_translation(
                    Vec2::from(ROAD_POSITIONS[i]).extend(BUTTON_Z),
                ))
                .insert(Visibility { is_visible: false })
                .id()
        }),
        buildings: enumerate([(); BUILDING_COUNT]).map(|(i, _)| {
            commands
                .spawn()
                .insert(BuildingSlot(None))
                .insert(BoardIndex(i))
                .insert(Transform::from_translation(
                    Vec2::from(BUILDING_POSITIONS[i]).extend(BUILDING_Z),
                ))
                .id()
        }),
        building_buttons: enumerate([(); BUILDING_COUNT]).map(|(i, _)| {
            commands
                .spawn()
                .insert(BuildingButton)
                .insert(BoardIndex(i))
                .insert(Transform::from_translation(
                    Vec2::from(BUILDING_POSITIONS[i]).extend(BUTTON_Z),
                ))
                .insert(Visibility { is_visible: false })
                .id()
        }),
    };

    commands.insert_resource(board);
}
