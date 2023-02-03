use std::mem::swap;
use macroquad::rand::RandomRange;

use crate::board::Board;

pub fn make_move(board: &mut Board) {
    let mut boards = board.get_next_boards();
    let i = RandomRange::gen_range(0, boards.len());

    swap(board, &mut boards[i]);
}