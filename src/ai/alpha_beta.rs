use std::time::Instant;
use macroquad::rand;

use crate::board::Board;
use crate::AIType;
use crate::board::{I_HEIGHT, I_WIDTH, MIN_SCORE};
use crate::transition_table::TransitionTable;

pub struct AlphaBeta {
    transposition_table: TransitionTable,
    nodes_explored: u128
}

impl AlphaBeta {
    pub fn new() -> Self {
        AlphaBeta { transposition_table: TransitionTable::new(), nodes_explored: 0}
    }

    fn negamax(&mut self, board: &Board, depth: u8, alpha: i8, beta: i8) -> i8 {
        self.nodes_explored += 1;

        // Check if game is drawn or if the search is at max depth
        if board.is_draw() || depth == 0{
            return 0;
        }
        
        // Get boards where the next move is not an immediate loss and if there
        // are no boards than return a negative evaluation
        let boards = board.get_next_non_losing_boards();
        if boards[0].is_none() {
            return -(I_WIDTH*I_HEIGHT - board.counter)/2
        }

        // Update alpha, lower bound, if alpha is below the lower bound and there 
        // is no beta larger than our max possible score.
        let mut a = alpha;
        let min = -(I_WIDTH*I_HEIGHT - 2 - board.counter)/2;
        if a < min {
            a = min;
            if a >= beta {
                return a;
            }
        }

        // Update beta, upper bound, if beta is greater than the max possible score and prune
        // if alpha is greater than beta.
        let max: i8 = match self.transposition_table.get(board.hash()) {
            Some(val) => val + MIN_SCORE - 1,
            None => (I_WIDTH*I_HEIGHT- 1 - board.counter)/2,
        };

        let mut b = beta;
        if beta > max {
            b = max;
            if alpha >= b { 
                return b; 
            }
        }

        // Run negamax
        for next_board in boards.iter().flatten() {
            let s = -self.negamax(next_board, depth - 1, -beta, -a);
            
            if s >= b { 
                return s; 
            }
            if s > a { 
                a = s; 
            }
        }

        // update transposition table
        self.transposition_table.set(board.hash(), a - MIN_SCORE + 1);

        // return alpha
        a
    }


    pub fn make_move(&mut self, board: &mut Board, max_depth: u8, ai_type: &AIType) {
        let boards = board.get_next_non_losing_boards();
        let mut game_ending_move_found = false;
        let mut index = 0;

        // If there is more than one possible none losing move, than we go
        // through the search process
        if boards[0].is_some() {
            let time = Instant::now();

            // Check if there are any moves that end the game. If so, use that and
            // avoid wasted computation in the search
            for (i, wrapped_board) in boards.iter().enumerate() {
                if let Some(b) = wrapped_board {
                    if b.is_game_over(b.bit_board[1]) {
                        index = i;
                        game_ending_move_found = true;
                        break;
                    }
                }
            }

            // Otherwise, go through the search process
            if !game_ending_move_found {
                let mut best_score = -(I_WIDTH*I_HEIGHT);
                let mut scores = Vec::new();

                // Evaluate possible moves with iterative deepening search
                let min = -(I_WIDTH*I_HEIGHT - board.counter)/2;
                let max = (I_WIDTH*I_HEIGHT + 1 - board.counter)/2;
                
                // Iterative deepening, starting at a reasonable depth
                for depth in (max_depth/3)..max_depth {
                    scores.clear();
                    for b in boards.iter().flatten() {
                        scores.push(-self.negamax(
                            b, 
                            depth, 
                            min,
                            max
                        ));
                    }
                }

                // RNG added to make easy and medium bots easier to defeat
                if *ai_type == AIType::Easy || *ai_type == AIType::Medium {
                    // Choose move probabilistically 
                    let sum = scores.iter().sum::<i8>() as f32;
                    let rand = rand::RandomRange::gen_range(0., 1.);
                    let mut current_probability = 0.;
                    for (i, s) in scores.iter().enumerate() {
                        current_probability += (*s as f32) / sum;
                        if current_probability >= rand {
                            index = i;
                            break;
                        }
                    }
                } else {
                    // Choose the best move
                    for (i, s) in scores.iter().enumerate() {
                        if *s > best_score {
                            best_score = *s;
                            index = i;
                        }
                    }
                }

                // Log some simple stats formatted for a markdown table
                let elapsed = time.elapsed();
                println!("| {} | {:?} | {} |", 
                    self.nodes_explored, 
                    elapsed, 
                    self.nodes_explored as f32  / (1000. * elapsed.as_secs_f32()));

                // Clear transposition table since it is no longer accurate with a 
                // depth limited approach
                self.transposition_table.reset();
                self.nodes_explored = 0;
            }

            // update the board
            std::mem::swap(board, &mut (boards[index].unwrap()));
        } else {
            // Else there isn't a non-losing, and we select a random board to 
            // keep the game going
            std::mem::swap(board, &mut board.get_next_boards()[0]);
        }
    }
}