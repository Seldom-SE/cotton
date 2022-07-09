use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerColor {
    Blue,
    Orange,
    Red,
    White,
}

impl From<PlayerColor> for String {
    fn from(color: PlayerColor) -> Self {
        match color {
            PlayerColor::Blue => "BLUE",
            PlayerColor::Orange => "ORANGE",
            PlayerColor::Red => "RED",
            PlayerColor::White => "WHITE",
        }
        .into()
    }
}

impl From<PlayerColor> for Color {
    fn from(color: PlayerColor) -> Self {
        match color {
            PlayerColor::Blue => Color::BLUE,
            PlayerColor::Orange => Color::rgb(1., 0.5, 0.),
            PlayerColor::Red => Color::RED,
            PlayerColor::White => default(),
        }
    }
}

pub const COLORS: [PlayerColor; 4] = [
    PlayerColor::Blue,
    PlayerColor::Orange,
    PlayerColor::Red,
    PlayerColor::White,
];
