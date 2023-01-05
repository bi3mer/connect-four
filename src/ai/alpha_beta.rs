// use std::collections::HashMap;

use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

use crate::board::Board;
use crate::AIType;
use crate::board::{F_HEIGHT, F_WIDTH};

pub struct AlphaBeta {
    // visited_states: HashMap<u128, f32>,
}

impl AlphaBeta {
    pub fn new() -> Self {
        // AlphaBeta { visited_states: HashMap::new() }
        AlphaBeta {}
    }

    fn negamax(&mut self, board: &Board, depth: u8) -> f32 {
        // Check if game is drawn
        if board.is_draw() {
            return 0.;
        }

        // Check if max depth reached
        if depth == 0 {
            return 0.;
        }
        
        // Check if there is a move that ends the game in the players favor.
        // Higher reward for moves that finish the game earlier.
        let boards = board.get_next_boards();
        let i = (board.counter & 1) as usize;
        for next_board in boards.iter() {
            if next_board.is_game_over(next_board.bit_board[i]) {
                return (F_WIDTH*F_HEIGHT+1. - (board.counter as f32)) / 2.;
            }
        }
        
        // Run negamax
        // https://en.wikipedia.org/wiki/Negamax#Negamax_with_alpha_beta_pruning
        let mut score: f32 = -10000.;
        
        for next_board in boards.iter() {
            let s = -self.negamax(next_board, depth - 1);
            if s > score {
                score = s;
            }
        }

        score
    }

    pub fn make_move(&mut self, board: &mut Board, max_depth: u8, ai_type: &AIType) {
        let mut rng = SmallRng::from_entropy();
        let mut boards = board.get_next_boards();
        let mut scores = Vec::new();

        for b in boards.iter() {

            let mut s = -self.negamax(b, max_depth);
            if *ai_type != AIType::Hard {
                s += rng.gen::<f32>()*0.;
            }

            scores.push(s);
        }

        let mut best_score = -100000.;
        let mut index = 0;
        for (i, s) in scores.iter().enumerate() {
            if *s > best_score {
                best_score = *s;
                index = i;
            }
        }

        // update the board
        std::mem::swap(board, &mut (boards[index]));
    }
}


 
        // if board.is_white_turn() {
        //     // Minimize since it is the player's turn
        //     score = 1000000000.;
        //     for cur_board in boards.iter() {
        //         let k = cur_board.hash();
        //         if self.visited_states.contains_key(&k) {
        //             score = score.min(*self.visited_states.get(&k).unwrap());
        //         } else {
        //             let board_score = self.ab(cur_board, depth - 1, a, b);
        //             self.visited_states.insert(k, board_score);
        //             score = score.min(board_score);

        //             b = b.min(score);
        //             if score <= alpha { break; }
        //         }

                
        //     }
        // } else {
        //     // Maximize since it is the AI's turn
        //     score = -1000000000.;
        //     for cur_board in boards.iter() {
        //         let k = cur_board.hash();
        //         if self.visited_states.contains_key(&k) {
        //             score = score.max(*self.visited_states.get(&k).unwrap());
        //         } else {
        //             let board_score = self.ab(cur_board, depth-1, a, b);
        //             self.visited_states.insert(k, board_score);
        //             score = score.max(board_score);

        //             a = a.max(score);
        //             if score >= beta { break; }
        //         }
        //     }
        // }
        // 
        // score