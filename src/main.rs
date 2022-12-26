use std::cmp::min;

use macroquad::prelude::*;
mod cell;
use cell::Cell;

mod board;
use board::*;

fn get_mouse_column(pos: (f32, f32), offset: f32, d: f32) -> Result<usize, ()> {
    let r = d/2.0;
    if pos.0 >= offset - r && 
       pos.0 <= offset + d*(F_WIDTH-1.0) + r &&
       pos.1 >= offset - r &&
       pos.1 <= offset + d*(F_HEIGHT-1.0) + r {
        let x = pos.0 - offset;
        Ok(((x+r) / d) as usize)
    } else {
        Err(())
    }
}

#[macroquad::main("Connect-Four")]
async fn main() {
    let mut board = Board::new();

    loop {
        if board.state == BoardState::Active {
            clear_background(BLACK);

            // get diameter of board based on current screen size
            let d = min(
                (screen_width() / (F_WIDTH + 4.0)) as i32, 
                (screen_height() / (F_HEIGHT + 4.0)) as i32
            ) as f32;
            let offset = d * 3.0;
            
            let mouse_pos = mouse_position();
            let mouse_col = get_mouse_column(mouse_pos, offset, d);
            match mouse_col {
                Ok(col_index) => {
                    // highlight the column the player is hovering over
                    draw_rectangle(
                        offset - d/2.0 + d*col_index as f32, 
                        offset - d/2.0, 
                        d, 
                        d*F_HEIGHT, 
                        Color { r: 0.188, g: 0.835, b: 0.784, a: 0.2 });

                    // player input to make a move on the board
                    if is_mouse_button_pressed(MouseButton::Left) {
                        board.make_move(col_index);
                    }
                },
                Err(_) => {},
            }
            
            // AI
            if board.turn == Cell::Red {
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

                draw_text(
                    format!("{}", i).as_str(),
                    x*d + offset, 
                    y*d + offset,
                    12.0,
                    BLUE
                );
            }
            
            // render the result if the game is over
            board.update_board_state();
            if board.state != BoardState::Active {
                let text = match board.state {
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