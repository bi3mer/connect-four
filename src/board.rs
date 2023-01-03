use std::{slice::Iter};
use crate::cell::Cell;

pub const S_WIDTH: usize = 7;
pub const S_HEIGHT: usize = 6;

pub const U_WIDTH: u8 = 7;
pub const U_HEIGHT: u8 = 6;

pub const F_WIDTH: f32 = 7.0;
pub const F_HEIGHT: f32 = 6.0;

#[derive(PartialEq, Copy, Clone)]
pub enum BoardState {
    WhiteWon = 0,
    RedWon,
    Draw,
    Active
}

/* 
https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
  6 13 20 27 34 41 48   55 62     Additional row
+---------------------+ 
| 5 12 19 26 33 40 47 | 54 61     top row
| 4 11 18 25 32 39 46 | 53 60
| 3 10 17 24 31 38 45 | 52 59
| 2  9 16 23 30 37 44 | 51 58
| 1  8 15 22 29 36 43 | 50 57
| 0  7 14 21 28 35 42 | 49 56 63  bottom row
+---------------------+

Using above as basis for this implementation.
*/

#[derive(Clone)]
pub struct Board {
    pub bit_board: [u64; 2],
    height: [u8; 7],
    counter: u8,
    pub state: BoardState
}

impl Board {
    pub fn new() -> Board {
        Board { 
            bit_board: [0; 2],
            height: [0, 7, 15, 24, 30, 35, 42],
            counter: 0,
            state: BoardState::Active 
        }
    }

    pub fn reset(&mut self) {
        self.bit_board[0] = 0;
        self.bit_board[1] = 0;
        self.height = [0, 7, 15, 24, 30, 35, 42];
        self.counter = 0;
        self.state = BoardState::Active;
    }

    pub fn get_next_boards(&self) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();
        for column in 0..S_WIDTH {
            let mut new_board = self.clone();
            if new_board.make_move(column) {
                boards.push(new_board);
            }
        }

        boards
    }

    pub fn make_move(&mut self, col: usize) -> bool {
        let h = self.height[col];
        if h >= 5 + (col as u8) *U_WIDTH {
            return false;
        }

        let move_pos = (1 as u64) << h;
        self.height[col] += 1;
        self.bit_board[(self.counter & 1) as usize] ^= move_pos; 
        self.counter += 1;
        
        true
    }

    pub fn is_game_over(&mut self, bit_board: u64) -> bool {
        if bit_board & bit_board >> 6 & bit_board >> 12 & bit_board >> 18 != 0 { return true; } // diagonal \
        if bit_board & bit_board >> 8 & bit_board >> 16 & bit_board >> 24 != 0 { return true; } // diagonal /
        if bit_board & bit_board >> 7 & bit_board >> 14 & bit_board >> 21 != 0 { return true; } // horizontal
        if bit_board & bit_board >> 1 & bit_board >>  2 & bit_board >>  3 != 0 { return true; } // vertical
        
        false 
    }
}

// TODO: implement iterator trait