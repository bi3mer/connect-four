use std::{slice::Iter};
use rand::{self, Rng, rngs::SmallRng, SeedableRng};
use crate::cell::Cell;

pub const U_WIDTH: usize = 7;
pub const U_HEIGHT: usize = 6;

pub const F_WIDTH: f32 = 7.0;
pub const F_HEIGHT: f32 = 6.0;

#[derive(PartialEq, Copy, Clone)]
pub enum BoardState {
    WhiteWon = 0,
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

    pub fn reset(&mut self) {
        self.turn = Cell::White;
        self.state = BoardState::Active;
        self.board.iter_mut().for_each(|e| { *e = Cell::Empty });
    }

    pub fn get(&self, index: usize) -> Cell {
        self.board[index]
    }

    pub fn set(&mut self, index: usize, val: Cell) {
        self.board[index] = val;
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

    fn check_indices(&mut self, len: usize, i_1: usize, i_2: usize, i_3: usize, i_4: usize) -> bool{
        if i_4 < len && i_3 < len && i_2 < len && i_1 < len && 
            self.board[i_1] != Cell::Empty &&
            self.board[i_1] == self.board[i_2] &&
            self.board[i_1] == self.board[i_3] &&
            self.board[i_1] == self.board[i_4] {
            if self.board[i_1] == Cell::Red {
                self.state = BoardState::RedWon;
            } else {
                self.state = BoardState::WhiteWon;
            }

            return true;
        }

        false
    }

    pub fn update_board_state(&mut self) -> Option<(usize, usize, usize, usize)> {
        let len = self.board.len();
        let mut empty_found = false;
        
        for row in 0..U_WIDTH {
            for col in 0..U_HEIGHT {
                let index = row + col * U_WIDTH;
                empty_found |= self.board[index] == Cell::Empty;
                let mut i_2;
                let mut i_3;
                let mut i_4;
                

                if row < U_WIDTH - 3 {
                    // check to the right
                    i_2 = row + 1 + col * U_WIDTH;
                    i_3 = row + 2 + col * U_WIDTH;
                    i_4 = row + 3 + col * U_WIDTH;
                    if self.check_indices(len, index, i_2, i_3, i_4) {
                        return Some((index, i_2, i_3, i_4));
                    }

                    // check diagonal down and to the right
                    i_2 = row + 1 + (col + 1) * U_WIDTH;
                    i_3 = row + 2 + (col + 2) * U_WIDTH;
                    i_4 = row + 3 + (col + 3) * U_WIDTH;
                    if self.check_indices(len, index, i_2, i_3, i_4) {
                        return Some((index, i_2, i_3, i_4));
                    }
                }

                // check diagonal down and to the left
                if row >= 3 {
                    i_2 = row - 1 + (col + 1) * U_WIDTH;
                    i_3 = row - 2 + (col + 2) * U_WIDTH;
                    i_4 = row - 3 + (col + 3) * U_WIDTH;
                    if self.check_indices(len, index, i_2, i_3, i_4) {
                        return Some((index, i_2, i_3, i_4));
                    }
                }

                // check straight down
                i_2 = row + (col + 1) * U_WIDTH; 
                i_3 = row + (col + 2) * U_WIDTH; 
                i_4 = row + (col + 3) * U_WIDTH;
                if self.check_indices(len, index, i_2, i_3, i_4) {
                    return Some((index, i_2, i_3, i_4));
                }
            }

            if self.state != BoardState::Active {
                break;
            }
        }

        // check if it is a draw
        if !empty_found && self.state == BoardState::Active {
            self.state = BoardState::Draw;
        }

        None
    }

    //////////////////////////// AI: Random ////////////////////////////
    pub fn random_ai_turn(&mut self) {
        let mut boards = self.get_next_boards();
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..boards.len());

        self.state = boards[i].state;
        std::mem::swap(&mut self.board, &mut (boards[i].board));
        self.turn = Cell::White;
    }

    //////////////////////////// AI: Minimax ////////////////////////////
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
        let mut score = 0.0;

        for b in boards.iter() {
            score += (1.0/(boards.len() as f32)) * b._minimax(depth-1);
        }

        score
    }
     
    pub fn minimax(&mut self, depth: u16, ai: &super::AI) {
        let mut rng = SmallRng::from_entropy();
        let mut boards = self.get_next_boards();
        let mut scores = Vec::new();
        for b in boards.iter() {
            let mut s = b._minimax(depth);
            if *ai != super::AI::Hard {
                s += rng.gen::<f32>();
            }

            scores.push(s);
        }

        let mut best_score = -10000.;
        let mut index = 0;
        for (i, s) in scores.iter().enumerate() {
            if *s > best_score {
                index = i;
                best_score = *s;
            }
        }

        // update the board
        std::mem::swap(&mut self.board, &mut (boards[index].board));
        self.turn = Cell::White;
        self.state = boards[index].state;
    }

    //////////////////////////// AI: AB Pruning ////////////////////////////
    

    //////////////////////////// AI: MCTS ////////////////////////////
}

