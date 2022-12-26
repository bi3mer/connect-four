use std::slice::Iter;
use crate::cell::Cell;

pub const U_WIDTH: usize = 7;
pub const U_HEIGHT: usize = 6;

pub const F_WIDTH: f32 = 7.0;
pub const F_HEIGHT: f32 = 6.0;

#[derive(PartialEq)]
pub enum BoardState {
    WhiteWon,
    RedWon,
    Draw,
    Active
}


pub struct Board {
    board: [Cell ; U_WIDTH*U_HEIGHT],
    pub turn: Cell,
    pub state: BoardState
}

impl Board {
    pub fn new() -> Board {
        Board { 
            board: [Cell::Empty; U_WIDTH*U_HEIGHT], 
            turn: Cell::White, state: 
            BoardState::Active 
        }
    }

    pub fn iter(&self) -> Iter<'_, Cell> {
        self.board.iter()
    }

    pub fn len(&self) -> usize {
        self.board.len()
    }

    pub fn get(&self, index: usize) -> Cell {
        self.board[index]
    }

    pub fn make_move(&mut self, column: usize) -> bool {
        let mut row: usize = 0;
        let mut index = column + row*U_WIDTH;
        if self.board[index] == Cell::Empty {
            row += 1;
            index = column + row*U_WIDTH;
            while index < self.board.len() && self.board[index] == Cell::Empty {
                row += 1;
                index = column + row*U_WIDTH;
            }

            self.board[column + (row-1)*U_WIDTH] = self.turn;

            if self.turn == Cell::Red {
                self.turn = Cell::White;
            } else if self.turn == Cell::White {
                self.turn = Cell::Red;
            }
            
            true
        } else {
            false
        }
    }

    pub fn update_board_state(&mut self) {
        self.state = BoardState::Active
    }
}