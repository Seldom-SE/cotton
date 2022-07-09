#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Blue,
    Orange,
    Red,
    White,
}

pub const COLORS: [Color; 4] = [Color::Blue, Color::Orange, Color::Red, Color::White];
