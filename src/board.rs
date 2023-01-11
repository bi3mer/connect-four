use crate::cell::Cell;

pub const S_WIDTH: usize = 7;
pub const S_HEIGHT: usize = 6;

pub const U_WIDTH: u8 = 7;
pub const U_HEIGHT: u8 = 6;

pub const F_WIDTH: f32 = 7.0;
pub const F_HEIGHT: f32 = 6.0;

pub const I_WIDTH: i8 = 7;
pub const I_HEIGHT: i8 = 6;

pub const COLUMN_ORDER: [usize; 7] = [3,4,2,5,1,6,0]; // search from the middle out

pub const MIN_SCORE: i8 = -(I_WIDTH*I_HEIGHT)/2 + 3;

/* 
https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
  6 13 20 27 34 41 48   55 62     Additional row
+---------------------+ 
| 5 12 19 26 33 40 47 | 54 61     top row
| 4 11 18 25 32 39 46 | 53 60
| 3 10 17 24 31 38 45 | 52 59
| 2  9 16 23 30 37 44 | 51 58
| 1  8 15 22 29 36 43 | 50 57
| 0  7 14 21 28 35 42 | 49 56 63  bottom row
+---------------------+

Using above as basis for this implementation.
*/

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub bit_board: [u64; 2], // 0 is player and 1 is the AI
    height: [u8; 7],
    pub counter: i8
}

impl Board {
    pub fn new() -> Board {
        Board { 
            bit_board: [0; 2],
            height: [0, 7, 14, 21, 28, 35, 42],
            counter: 0,
        }
    }

    pub fn reset(&mut self) {
        self.bit_board[0] = 0;
        self.bit_board[1] = 0;
        self.height = [0, 7, 14, 21, 28, 35, 42];
        self.counter = 0;
    }

    fn loses_next_turn(&mut self) -> bool {
        let index = (self.counter & 1) as usize;
        for column in COLUMN_ORDER {
            if self.make_move(column) {
                let game_over = self.is_game_over(self.bit_board[index]);
                self.undo_move(column);

                if game_over {
                    return true;
                }
            }
        }

        false
    }

    // Return all possible next boards but will not return boards where
    // the next move can result in a guaranteed loss.
    pub fn get_next_non_losing_boards(&self) -> [Option<Board>; S_WIDTH] {
        let mut boards = [None; S_WIDTH];
        let mut i = 0;

        for column in COLUMN_ORDER {
            let mut new_board = *self;
            let new_board_is_valid = new_board.make_move(column);
            // if new_board_is_valid && new_board.opponent_winning_moves() == 0 {
            //     boards[i] = Some(new_board);
            //     i += 1;
            // }

            if new_board_is_valid && !new_board.loses_next_turn() {
                boards[i] = Some(new_board);
                i += 1;

                let old = !new_board.loses_next_turn();
                let new = new_board.opponent_winning_moves() == 0;
                if old != new {
                    println!("=================================================\n\n");
                    self.print_self();
                    new_board.print_self();
                    println!("old: {:?}", old);
                    println!("new: {:?}", new);
                    println!("{:b}", new_board.opponent_winning_moves());
                    panic!();
                }
            }

        }

        boards
    }

    // Return all possible next boards
    pub fn get_next_boards(&self) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();
        for column in COLUMN_ORDER {
            let mut new_board = *self;
            if new_board.make_move(column) {
                boards.push(new_board);
            }
        }

        boards
    }

    pub fn is_white_turn(&self) -> bool{
        self.counter % 2 == 0
    }

    pub fn make_move(&mut self, col: usize) -> bool {
        let h = self.height[col];
        if h >= 6 + (col as u8) * U_WIDTH {
            return false;
        }

        let move_pos = (1_u64) << h;
        self.bit_board[(self.counter & 1) as usize] ^= move_pos; 
        self.height[col] += 1;
        self.counter += 1;
        
        true
    }

    pub fn undo_move(&mut self, col: usize) {
        self.counter -= 1;
        self.height[col] -= 1;
        let move_pos = 1_u64 << (self.height[col] as i64);
        self.bit_board[(self.counter & 1) as usize] ^= move_pos;
    }

    // https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md#are-there-four-in-a-row
    pub fn is_game_over(&self, bit_board: u64) -> bool {
        let diag_1 = bit_board & (bit_board >> 6); // diagonal \
        let diag_2 = bit_board & (bit_board >> 8);
        let horizontal = bit_board & (bit_board >> 7);
        let vertical = bit_board & (bit_board >> 1);

        diag_1 & (diag_1 >> 12) != 0 ||
        diag_2 & (diag_2 >> 16) != 0 ||
        horizontal & (horizontal >> 14) != 0 ||
        vertical & (vertical >> 2) != 0
    }

    pub fn is_draw(&self) -> bool {
        self.counter == 42 // U_WIDTH * U_HEIGHT = 6 * 7 = 42
    }

    pub fn hash(&self) -> u64 {
        let index = (self.counter & 1) as usize;
        let index_2 = ((self.counter+1) & 1) as usize;
        self.bit_board[index] + (self.bit_board[index] | self.bit_board[index_2])
    }

    pub fn winning_moves(&self) -> u64 {
        let index = (self.counter & 1) as usize;
        let index_2 = ((self.counter+1) & 1) as usize;
        self.compute_winning_positions(self.bit_board[index], self.bit_board[index_2])
    }

    pub fn opponent_winning_moves(&self) -> u64 {
        let index = (self.counter & 1) as usize;
        let index_2 = ((self.counter+1) & 1) as usize;
        self.compute_winning_positions(self.bit_board[index_2], self.bit_board[index])
    }

    // Based on:
    // https://github.com/PascalPons/connect4/blob/7ed79f6e6315c0f95ee35194520dd615eddbd27d/position.hpp#L272
    fn compute_winning_positions(&self, bit_board: u64, bit_board_opponent: u64) -> u64 {
        // vertical
        let mut r = (bit_board << 1) & (bit_board << 2) & (bit_board << 3);

        //horizontal
        let mut p = (bit_board << (U_HEIGHT+1)) & (bit_board << 2*(U_HEIGHT+1));
        r |= p & (bit_board << 3*(U_HEIGHT+1));
        r |= p & (bit_board >> (U_HEIGHT+1));
        p = (bit_board >> (U_HEIGHT+1)) & (bit_board >> 2*(U_HEIGHT+1));
        r |= p & (bit_board << (U_HEIGHT+1));
        r |= p & (bit_board >> 3*(U_HEIGHT+1));

        //diagonal 1
        p = (bit_board << U_HEIGHT) & (bit_board << 2*U_HEIGHT);
        r |= p & (bit_board << 3*U_HEIGHT);
        r |= p & (bit_board >> U_HEIGHT);
        p = (bit_board >> U_HEIGHT) & (bit_board >> 2*U_HEIGHT);
        r |= p & (bit_board << U_HEIGHT);
        r |= p & (bit_board >> 3*U_HEIGHT);

        //diagonal 2
        p = (bit_board << (U_HEIGHT+2)) & (bit_board << 2*(U_HEIGHT+2));
        r |= p & (bit_board << 3*(U_HEIGHT+2));
        r |= p & (bit_board >> (U_HEIGHT+2));
        p = (bit_board >> (U_HEIGHT+2)) & (bit_board >> 2*(U_HEIGHT+2));
        r |= p & (bit_board << (U_HEIGHT+2));
        r |= p & (bit_board >> 3*(U_HEIGHT+2));

        r & !bit_board_opponent
    }

    // refer to board above for the for magic numbers to make sense
    pub fn get_cells(&self) -> [Cell; S_WIDTH*S_HEIGHT] {
        let mut board = [Cell::Empty; S_WIDTH*S_HEIGHT];
        let mut i = 0;
        for row in (0..6).rev() {
            for col in 0..S_WIDTH {
                let index = row + col*S_WIDTH;
                if self.bit_board[0] & (1 << index) != 0 {
                    board[i] = Cell::White;
                } else if self.bit_board[1] & (1 << index) != 0 {
                    board[i] = Cell::Red;
                } 
                i += 1;
            }
        }
        
        board
    }

    // used for debugging
    #[warn(dead_code)]
    pub fn print_self(&self) {
        println!("\n!!!!!!!");
        for (i, cell) in self.get_cells().iter().enumerate() {
            if i != 0 && i % S_WIDTH == 0 {
                println!();
            }

            match cell {
                Cell::Empty => { print!("-"); },
                Cell::White => { print!("X"); },
                Cell::Red =>   { print!("O"); }
            }
        }

        println!("\n!!!!!!!\n\n");
    }
}
