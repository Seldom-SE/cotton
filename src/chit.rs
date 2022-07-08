use bevy::prelude::*;

use crate::{image::UpdateImages, random::Shuffle};

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

#[derive(Clone, Component, Copy, Deref)]
pub struct ChitSlot(pub Option<Chit>);

impl UpdateImages for ChitSlot {
    fn image(self) -> Option<&'static str> {
        match *self {
            None => None,
            Some(Chit(2)) => Some("chit_2.png"),
            Some(Chit(3)) => Some("chit_3.png"),
            Some(Chit(4)) => Some("chit_4.png"),
            Some(Chit(5)) => Some("chit_5.png"),
            Some(Chit(6)) => Some("chit_6.png"),
            Some(Chit(8)) => Some("chit_8.png"),
            Some(Chit(9)) => Some("chit_9.png"),
            Some(Chit(10)) => Some("chit_10.png"),
            Some(Chit(11)) => Some("chit_11.png"),
            Some(Chit(12)) => Some("chit_12.png"),
            Some(Chit(value)) => panic!("invalid chit with value: {value}"),
        }
    }
}
