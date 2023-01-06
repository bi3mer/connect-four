use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

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
        let max: i8 = match self.transposition_table.get(board.hash()) {
            Some(val) => val + MIN_SCORE - 1,
            None => (I_WIDTH*I_HEIGHT- 1 - board.counter)/2,
        };

        let mut b = beta;
        if beta > max {
            b = max;
            if alpha >= b { return b; }
        }
        
        // Run negamax
        let mut a = alpha;
        for next_board in boards.iter() {
            let s = -self.negamax(next_board, depth - 1, -beta, -a);
            if s >= b { return s; }
            if s > a { a = s; }
        }

        self.transposition_table.set(board.hash(), a - MIN_SCORE + 1);
        a
    }

    pub fn iterative_deepening(&mut self, board: &Board, max_depth: u8) -> i8 {
        let max = (I_WIDTH*I_HEIGHT + 1 - board.counter)/2;
        let min = -(I_WIDTH*I_HEIGHT - board.counter)/2;

        let mut score = 0;
        for depth in 1..max_depth {
            score = self.negamax(board, depth, min, max);
        }

        score
    }

    pub fn make_move(&mut self, board: &mut Board, max_depth: u8, ai_type: &AIType) {
        let mut boards = board.get_next_boards();
        let mut game_ending_move_found = false;
        let mut index = 0;

        // Check if there are any moves that end the game. If so, use that and
        // avoid wasted computation in the search
        for (i, b) in boards.iter().enumerate() {
            if b.is_game_over(b.bit_board[1]) {
                index = i;
                game_ending_move_found = true;
                break;
            }
        }

        // Otherwise, go through the search process
        if !game_ending_move_found {
            let mut rng = SmallRng::from_entropy();
            let mut best_score = -(I_WIDTH*I_HEIGHT);
            let mut scores = Vec::new();

            // Evaluate possible moves with iterative deepening search
            for b in boards.iter() {
                // let max = (I_WIDTH*I_HEIGHT + 1 - board.counter)/2;
                // let min = -(I_WIDTH*I_HEIGHT - board.counter)/2;
                // let mut s = -self.negamax(
                //     b, 
                //     max_depth, 
                //     min,
                //     max
                // );
                let mut s = -self.iterative_deepening(b, max_depth);
                
                // RNG added to make easy and medium bots easier to defeat
                if *ai_type == AIType::Easy || *ai_type == AIType::Medium {
                    s += rng.gen::<i8>();
                }
                
                scores.push(s);
            }

            // Choose the best move
            for (i, s) in scores.iter().enumerate() {
                if *s > best_score {
                    best_score = *s;
                    index = i;
                }
            }

            // Clear transposition table since it is no longer accurate with a 
            // depth limited approach
            println!("Explored {} nodes", self.nodes_explored);
            self.transposition_table.reset();
            self.nodes_explored = 0;
        }

        // update the board
        std::mem::swap(board, &mut (boards[index]));
    }
}