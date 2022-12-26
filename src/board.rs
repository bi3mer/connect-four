use std::slice::Iter;
use rand::{self, Rng};
use crate::cell::Cell;

pub const U_WIDTH: usize = 7;
pub const U_HEIGHT: usize = 6;

pub const F_WIDTH: f32 = 7.0;
pub const F_HEIGHT: f32 = 6.0;

#[derive(PartialEq, Copy, Clone)]
pub enum BoardState {
    WhiteWon,
    RedWon,
    Draw,
    Active
}


#[derive(Clone)]
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

    pub fn get_next_boards(&self) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();
        for column in 0..U_WIDTH {
            let mut new_board = self.clone();
            
            if new_board.make_move(column) {
                boards.push(new_board);
            }
        }

        boards
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

    fn check_indices(&mut self, len: usize, i_1: usize, i_2: usize, i_3: usize, i_4: usize) {
        if i_4 < len && i_3 < len && i_2 < len && i_1 < len && 
            self.board[i_1] != Cell::Empty &&
            self.board[i_1] == self.board[i_2] &&
            self.board[i_1] == self.board[i_3] &&
            self.board[i_1] == self.board[i_4] 
            {
            if self.board[i_1] == Cell::Red {
                self.state = BoardState::RedWon;
            } else {
                self.state = BoardState::WhiteWon;
            }
        }
    }

    pub fn update_board_state(&mut self) {
        let len = self.board.len();
        for row in 0..U_WIDTH {
            for col in 0..U_HEIGHT {
                // check to the right
                let mut i_1 = row + col * U_WIDTH;
                let mut i_2 = row + 1 + col * U_WIDTH;
                let mut i_3 = row + 2 + col * U_WIDTH;
                let mut i_4 = row + 3 + col * U_WIDTH;
                self.check_indices(len, i_1, i_2, i_3, i_4);

                // check diagonal down and to the right
                i_1 = row + col * U_WIDTH;
                i_2 = row + 1 + (col + 1) * U_WIDTH;
                i_3 = row + 2 + (col + 2) * U_WIDTH;
                i_4 = row + 3 + (col + 3) * U_WIDTH;
                self.check_indices(len, i_1, i_2, i_3, i_4);

                // check diagonal down and to the left
                if row >= 3 {
                    i_1 = row + col * U_WIDTH;
                    i_2 = row - 1 + (col + 1) * U_WIDTH;
                    i_3 = row - 2 + (col + 2) * U_WIDTH;
                    i_4 = row - 3 + (col + 3) * U_WIDTH;
                    self.check_indices(len, i_1, i_2, i_3, i_4);
                }

                // check straight down
                i_1 = row + col * U_WIDTH;
                i_2 = row + (col + 1) * U_WIDTH;
                i_3 = row + (col + 2) * U_WIDTH;
                i_4 = row + (col + 3) * U_WIDTH;
                self.check_indices(len, i_1, i_2, i_3, i_4);

            }

            if self.state != BoardState::Active {
                break;
            }
        }
    }

    pub fn random_ai_turn(&mut self) {
        let mut boards = self.get_next_boards();
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..boards.len());

        std::mem::swap(&mut self.board, &mut (boards[i].board));
        self.turn = Cell::White;
        self.update_board_state();
    }
}

