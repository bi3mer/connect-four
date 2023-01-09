use rand::{self, Rng};
use std::mem::swap;

use crate::board::Board;

pub fn make_move(board: &mut Board) {
    let mut boards = board.get_next_non_losing_boards();
    if boards.len() == 0 {
        boards = board.get_next_boards();
    }
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..boards.len());

    swap(board, &mut boards[i]);
}