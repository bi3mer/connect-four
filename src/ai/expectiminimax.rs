use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

use crate::board::{Board, BoardState};
use crate::cell::Cell;
use crate::AIType;

// fn expecti_minimax(board: &Board, depth: u16) -> f32 {
//     if board.state != BoardState::Active {
//         if board.state == BoardState::Draw {
//             return 1.0;
//         } else if board.state == BoardState::RedWon {
//             return 2.0;
//         } else {
//             return -2.0;
//         }
//     }

//     if depth == 0 {
//         return 1.0;
//     }

//     let boards = board.get_next_boards();
//     let mut score = 0.0;

//     for b in boards.iter() {
//         score += (1.0/(boards.len() as f32)) * expecti_minimax(b, depth-1);
//     }

//     score
// }
 
pub fn make_move(board: &mut Board, depth: u16, ai_type: &AIType) {
//     let mut rng = SmallRng::from_entropy();
//     let mut boards = board.get_next_boards();
//     let mut scores = Vec::new();
//     for b in boards.iter() {
//         let mut s = expecti_minimax(b, depth);
//         if *ai_type != AIType::Hard {
//             s += rng.gen::<f32>();
//         }

//         scores.push(s);
//     }

//     let mut best_score = -10000.;
//     let mut index = 0;
//     for (i, s) in scores.iter().enumerate() {
//         if *s > best_score {
//             index = i;
//             best_score = *s;
//         }
//     }

//     // update the board
//     std::mem::swap(&mut board.board, &mut (boards[index].board));
//     board.turn = Cell::White;
//     board.state = boards[index].state;
}