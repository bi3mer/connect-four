use std::cmp::min;

use macroquad::prelude::*;

const U_WIDTH: usize = 7;
const U_HEIGHT: usize = 6;

const F_WIDTH: f32 = 7.0;
const F_HEIGHT: f32 = 6.0;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    White,
    Red
}

#[derive(PartialEq)]
enum BoardState {
    WhiteWon,
    RedWon,
    Draw,
    Active
}

impl Cell {
    fn to_color(&self) -> Color {
        match self {
            Cell::Empty => Color::new(0.75, 0.0, 0.05, 0.2),
            Cell::White => macroquad::prelude::WHITE,
            Cell::Red => RED,
        }
    }
}

fn get_board_state(board: &[Cell]) -> BoardState {
    BoardState::Active
}

fn get_mouse_column(pos: (f32, f32), offset: f32, d: f32) -> Result<usize, ()> {
    let r = d/2.0;
    if pos.0 >= offset - r && 
       pos.0 <= offset + d*(F_WIDTH-1.0) + r &&
       pos.1 >= offset - r &&
       pos.1 <= offset + d*(F_HEIGHT-1.0) + r {
        Ok(1)
    } else {
        Err(())
    }
}

#[macroquad::main("Connect-Four")]
async fn main() {
    let mut board = [Cell::Empty; U_WIDTH*U_HEIGHT];
    let mut board_state = BoardState::Active;

    loop {
        if board_state == BoardState::Active {
            clear_background(BLACK);

            // get diameter of board based on current screen size
            let d = min(
                (screen_width() / (F_WIDTH + 4.0)) as i32, 
                (screen_height() / (F_HEIGHT + 4.0)) as i32
            ) as f32;
            let offset = d * 3.0;

            // highlight the column the player is hovering over
            let mouse_pos = mouse_position();
            let mouse_col = get_mouse_column(mouse_pos, offset, d);
            match mouse_col {
                Ok(col_index) => {
                    draw_rectangle(
                        offset - d/2.0 + d*col_index as f32, 
                        offset - d/2.0, 
                        d, 
                        d*F_HEIGHT, 
                        Color { r: 0.188, g: 0.835, b: 0.784, a: 0.2 });
                },
                Err(_) => {},
            }
                

            // player input to make a move on the board
            let mut ai_should_play = false;
            if is_mouse_button_released(MouseButton::Left) {
                if mouse_pos.0 >= offset - d/2.0 && 
                   mouse_pos.0 <= F_WIDTH * d + offset + d/2.0 && 
                   mouse_pos.1 >= offset - d/2.0 && 
                   mouse_pos.1 <= F_WIDTH * d + offset + d/2.0 {
                    ai_should_play = true;
                }
            }
            
            // AI
            if ai_should_play {
                println!("AI not implemented yet!");
            }


            // render the board
            for (i, cell) in board.iter().enumerate() {
                let x = (i % U_WIDTH) as f32;
                let y = (i / U_WIDTH) as f32;
                draw_circle(
                    x*d + offset, 
                    y*d + offset, 
                    d/2.0, 
                    cell.to_color());

                // draw_text(
                //     format!("{}", i).as_str(),
                //     x*d + offset, 
                //     y*d + offset,
                //     12.0,
                //     WHITE
                // );
            }
            
            // render the result
            board_state = get_board_state(&board);
            if board_state != BoardState::Active {
                let text = match board_state {
                    BoardState::WhiteWon => "You won!\nPress 'r' to play again.",
                    BoardState::RedWon => "The AI won!\nPress 'r' to play again.",
                    BoardState::Draw => "Draw!\nPress 'r' to play again.",
                    BoardState::Active => "GameState should not be 'Active' if the game is over. Contact admin."
                };

                draw_text(
                    text, 
                    screen_width() / 2.0, 
                    screen_height() - d, 
                    d-2.0, 
                    WHITE);
            }
        }

        next_frame().await
    }
}