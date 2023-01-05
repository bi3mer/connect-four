use std::collections::HashMap;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

use crate::board::Board;
use crate::AIType;
use crate::board::{I_HEIGHT, I_WIDTH, MIN_SCORE};

pub struct AlphaBeta {
    transposition_table: HashMap<u128, i8>,
}

impl AlphaBeta {
    pub fn new() -> Self {
        AlphaBeta { transposition_table: HashMap::new() }
    }

    fn negamax(&mut self, board: &Board, depth: u8, alpha: i8, beta: i8) -> i8 {
        // Check if game is drawn
        if board.is_draw() {
            return 0;
        }

        // Check if max depth reached
        if depth == 0 {
            return 0;
        }
        
        // Check if there is a move that ends the game in the players favor.
        // Higher reward for moves that finish the game earlier.
        let boards = board.get_next_boards();
        let i = (board.counter & 1) as usize;
        for next_board in boards.iter() {
            if next_board.is_game_over(next_board.bit_board[i]) {
                return (I_WIDTH*I_HEIGHT + 1 - board.counter) / 2;
            }
        }

        // update beta if beta is greater than the max possible score and prune
        // if alpha is greater than beta.
        let max: i8 = match self.transposition_table.get(&board.hash()) {
            Some(val) => val + MIN_SCORE - 1,
            None => (I_WIDTH*I_HEIGHT- 1 - board.counter)/2,
        };

        let mut b = beta;
        if beta > max {
            b = max;
            if alpha >= b { return b; }
        }
        
        // Run negamax
        // https://en.wikipedia.org/wiki/Negamax#Negamax_with_alpha_beta_pruning
        // let mut score: f32 = -10000.;
        let mut a = alpha;
        
        for next_board in boards.iter() {
            let s = -self.negamax(next_board, depth - 1, -beta, -a);
            if s >= b { return s; }
            if s > a { a = s; }
        }

        self.transposition_table.insert(board.hash(), a - MIN_SCORE + 1);
        a
    }

    pub fn make_move(&mut self, board: &mut Board, max_depth: u8, ai_type: &AIType) {
        let mut rng = SmallRng::from_entropy();
        let mut boards = board.get_next_boards();
        let mut scores = Vec::new();

        // evaluate possible moves
        for b in boards.iter() {
            let mut s = -self.negamax(
                b, 
                max_depth, 
                -(I_WIDTH*I_HEIGHT)/2,
                (I_WIDTH*I_HEIGHT)/2
            );

            // RNG added to make easy and medium bots easier to defeat
            if *ai_type == AIType::Easy || *ai_type == AIType::Medium {
                s += rng.gen::<i8>();
            }

            scores.push(s);
        }

        // clear transposition table since it is no longer accurate with a 
        // depth limited approach
        self.transposition_table.clear();

        // chose best move
        let mut best_score = -(I_WIDTH*I_HEIGHT);
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