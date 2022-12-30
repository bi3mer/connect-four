use rand::{self, Rng};
use std::mem::swap;

use crate::board::Board;
use crate::cell::Cell;

pub fn make_move(board: &mut Board) {
    let mut boards = board.get_next_boards();
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..boards.len());

    board.state = boards[i].state;
    swap(&mut board.board, &mut (boards[i].board));
    board.turn = Cell::White;
}