use macroquad::prelude::{Color, WHITE, RED};

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    White,
    Red
}

impl Cell {
    pub fn to_color(&self) -> Color {
        match self {
            Cell::Empty => Color::new(0.75, 0.0, 0.05, 0.2),
            Cell::White => WHITE,
            Cell::Red => RED,
        }
    }
}