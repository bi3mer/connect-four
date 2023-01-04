use std::collections::HashMap;
use std::cmp::{min, max};

use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

use crate::board::Board;
use crate::AIType;

fn minimax(board: &Board, depth: u8, visited_states: &mut HashMap<u128, f32>) -> f32 {
    if board.is_white_turn() {
        if board.is_game_over(board.bit_board[0]) {
            return -2.0;
        }
    } else if board.is_game_over(board.bit_board[1]) {
        return 2.0;
    } else if board.is_draw() {
        return 1.0;
    }

    if depth == 0 {
        return 0.0;
    }

    let boards = board.get_next_boards();
    let mut score: f32;

    if board.is_white_turn() {
        // Minimize since it is the player's turn
        score = 100000.;
        for b in boards.iter() {
            let k = b.hash();
            if visited_states.contains_key(&k) {
                score = score.min(*visited_states.get(&k).unwrap());
            } else {
                let board_score = minimax(b, depth - 1, visited_states);
                visited_states.insert(k, board_score);
                score = score.min(board_score);
            }
        }
    } else {
        // Maximize since it is the AI's turn
        score = -100000.;
        for b in boards.iter() {
            let k = b.hash();
            if visited_states.contains_key(&k) {
                score = score.max(*visited_states.get(&k).unwrap());
            } else {
                let board_score = minimax(b, depth-1, visited_states);
                visited_states.insert(k, board_score);
                score = score.max(board_score);
            }
        }
    }

    score
}
 
pub fn make_move(board: &mut Board, max_depth: u8, ai_type: &AIType) {
    let mut rng = SmallRng::from_entropy();
    let mut boards = board.get_next_boards();
    let mut scores = Vec::new();
    for b in boards.iter() {
        let mut visited_states = HashMap::new();

        let mut s = minimax(b, max_depth, &mut visited_states);
        if *ai_type != AIType::Hard {
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
    std::mem::swap(board, &mut (boards[index]));
}