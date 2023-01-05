use crate::cell::Cell;

pub const S_WIDTH: usize = 7;
pub const S_HEIGHT: usize = 6;

pub const U_WIDTH: u8 = 7;
// pub const U_HEIGHT: u8 = 6;

pub const F_WIDTH: f32 = 7.0;
pub const F_HEIGHT: f32 = 6.0;

pub const DIRECTIONS: [u8; 4] = [1, 7, 6, 8];
pub const COLUMN_ORDER: [usize; 7] = [3,4,2,5,1,6,0]; // search from the middle out

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

#[derive(Clone)]
pub struct Board {
    pub bit_board: [u64; 2], // 0 is player and 1 is the AI
    height: [u8; 7],
    pub counter: u8
}

impl Board {
    pub fn new() -> Board {
        Board { 
            bit_board: [0; 2],
            height: [0, 7, 14, 21, 28, 35, 42],
            counter: 0
        }
    }

    pub fn reset(&mut self) {
        self.bit_board[0] = 0;
        self.bit_board[1] = 0;
        self.height = [0, 7, 14, 21, 28, 35, 42];
        self.counter = 0;
    }

    pub fn get_next_boards(&self) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();
        for column in COLUMN_ORDER {
            let mut new_board = self.clone();
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
        self.height[col] += 1;
        self.bit_board[(self.counter & 1) as usize] ^= move_pos; 
        self.counter += 1;
        
        true
    }

    // https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md#are-there-four-in-a-row
    pub fn is_game_over(&self, bit_board: u64) -> bool {
        let mut bb: u64;
        for d in DIRECTIONS.iter() {
            bb = bit_board & (bit_board >> d);
            if (bb & (bb >> (2*d))) != 0 {
                return true;
            }
        }

        false 
    }

    pub fn is_draw(&self) -> bool {
        // U_WIDTH * U_HEIGHT = 8 * 7 = 42
        self.counter == 42 
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

    // pub fn hash(&self) -> u128 {
    //     let h: u128 = 0;
    //     (h << 64) | (self.bit_board[0] as u128) | (h >> 64) | ((self.bit_board[1] as u128) << 64)
    // }
}