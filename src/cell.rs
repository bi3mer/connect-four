use macroquad::prelude::{Color, WHITE, RED};

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    White,
    Red,
    // RedVictory,
    // WhiteVictory
}

impl Cell {
    pub fn to_color(self) -> Color {
        match self {
            Cell::Empty => Color::new(0.75, 0.0, 0.05, 0.2),
            Cell::White => WHITE,
            Cell::Red => RED,
            // Cell::WhiteVictory => Color::new(0.8, 0.8, 0.8, 0.9),
            // Cell::RedVictory => Color::new(1.0, 0.3, 0.3, 0.5),
        }
    }
}