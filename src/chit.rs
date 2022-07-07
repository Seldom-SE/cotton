use bevy::prelude::*;

use crate::random::Shuffle;

#[derive(Clone, Copy)]
pub struct Chit(u8);

static CHITS: &[Chit] = &[
    Chit(2),
    Chit(3),
    Chit(4),
    Chit(5),
    Chit(6),
    Chit(8),
    Chit(9),
    Chit(10),
    Chit(11),
    Chit(12),
];

impl Shuffle for Chit {
    fn pool() -> &'static [Self] {
        CHITS
    }

    fn weight(self) -> f32 {
        match self {
            Chit(2) | Chit(12) => 1.,
            Chit(3) | Chit(4) | Chit(5) | Chit(6) | Chit(8) | Chit(9) | Chit(10) | Chit(11) => 2.,
            Chit(value) => panic!("invalid chit with value: {value}"),
        }
    }
}

#[derive(Component)]
pub struct ChitSlot(pub Option<Chit>);
