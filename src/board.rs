use std::slice::Iter;
use std::cmp::{max, min};
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
            turn: Cell::White,
            state: BoardState::Active 
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

            self.update_board_state();
            
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
            self.board[i_1] == self.board[i_4] {
            
            println!("Game over! {},{},{},{}", i_1, i_2, i_3, i_4);
            if self.board[i_1] == Cell::Red {
                self.state = BoardState::RedWon;
            } else {
                self.state = BoardState::WhiteWon;
            }
        }
    }

    pub fn update_board_state(&mut self) {
        let len = self.board.len();
        let mut empty_found = false;
        
        for row in 0..U_WIDTH {
            for col in 0..U_HEIGHT {
                let index = row + col * U_WIDTH;
                empty_found |= self.board[index] == Cell::Empty;

                // check to the right
                self.check_indices(
                    len, 
                    index, 
                    row + 1 + col * U_WIDTH, 
                    row + 2 + col * U_WIDTH, 
                    row + 3 + col * U_WIDTH);

                // check diagonal down and to the right
                self.check_indices(
                    len, 
                    index, 
                    row + 1 + (col + 1) * U_WIDTH, 
                    row + 2 + (col + 2) * U_WIDTH, 
                    row + 3 + (col + 3) * U_WIDTH);

                // check diagonal down and to the left
                if row >= 3 {
                    self.check_indices(
                        len, 
                        index, 
                        row - 1 + (col + 1) * U_WIDTH, 
                        row - 2 + (col + 2) * U_WIDTH, 
                        row - 3 + (col + 3) * U_WIDTH);
                }

                // check straight down
                self.check_indices(
                    len, 
                    index, 
                    row + (col + 1) * U_WIDTH, 
                    row + (col + 2) * U_WIDTH, 
                    row + (col + 3) * U_WIDTH);
            }

            if self.state != BoardState::Active {
                println!("Game over!");
                break;
            }
        }

        // check if it is a draw
        if !empty_found && self.state == BoardState::Active {
            self.state = BoardState::Draw;
        }
    }

    pub fn random_ai_turn(&mut self) {
        let mut boards = self.get_next_boards();
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..boards.len());

        self.state = boards[i].state;
        std::mem::swap(&mut self.board, &mut (boards[i].board));
        self.turn = Cell::White;
    }

    fn _minimax(&self, depth: u16) -> f32 {
        if self.state != BoardState::Active {
            if self.state == BoardState::Draw {
                return 1.0;
            } else if self.state == BoardState::RedWon {
                return 2.0;
            } else {
                return -2.0;
            }
        }

        if depth == 0 {
            return 1.0;
        }

        let boards = self.get_next_boards();
        let mut score: f32 = 0.0;
        if self.turn == Cell::White {
            // minimize, players turn
            score = 100000000.0;
            for b in boards.iter() {
                score -= (1.0/(boards.len() as f32)) * b._minimax(depth-1);
                // score = score.min(b._minimax(depth-1));
            }
        } else {
            // maximize, ai's turn
            // score = -100000000.0;
            for b in boards.iter() {
                score -= (1.0/(boards.len() as f32)) * b._minimax(depth-1);
                // score = score.max(b._minimax(depth-1));
            }
        }

        score
    }
     
    pub fn minimax(&mut self, depth: u16) {
        let mut boards = self.get_next_boards();
        let mut best_score = -100000.0;
        let mut index = 0;
        for (i, b)in boards.iter().enumerate() {
            let score = b._minimax(depth);
            if score > best_score {
                best_score = score;
                index = i;
            }
        }

        // update the board
        std::mem::swap(&mut self.board, &mut (boards[index].board));
        self.turn = Cell::White;
        self.update_board_state();
    }
}

