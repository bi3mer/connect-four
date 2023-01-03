use macroquad::prelude::*;
use std::cmp::min;

use crate::AIType::{self, *};
use crate::{ai, board::*, cell::Cell};

use super::scene_id::SceneId::{self, *};
use super::scene_trait::Scene;


pub struct GameScene {
    board: Board
}



impl GameScene {
    pub fn new() -> Self {
        GameScene {
            board: Board::new()
        }
    }

    fn get_mouse_column(&self, pos: (f32, f32), offset: f32, d: f32) -> Option<usize> {
        let r = d/2.0;
        if pos.0 >= offset - r && 
           pos.0 <= offset + d*(F_WIDTH-1.0) + r &&
           pos.1 >= offset - r &&
           pos.1 <= offset + d*(F_HEIGHT-1.0) + r {
            let x = pos.0 - offset;
            Some(((x+r) / d) as usize)
        } else {
            None
        }
    }
}

impl Scene for GameScene {
    fn update(&mut self, ai: &mut AIType) -> SceneId {
        let mut target_scene = Game;

        // get diameter of board based on current screen size
        let d = min(
            (screen_width() / (F_WIDTH + 4.0)) as i32, 
            (screen_height() / (F_HEIGHT + 4.0)) as i32
        ) as f32;
        let offset = d * 3.0;

        if self.board.state == BoardState::Active {
            let mouse_pos = mouse_position();
            let mouse_col = self.get_mouse_column(mouse_pos, offset, d);
            if let Some(col_index) = mouse_col {
                // highlight the column the player is hovering over
                draw_rectangle(
                    offset - d/2.0 + d*col_index as f32, 
                    offset - d/2.0, 
                    d, 
                    d*F_HEIGHT, 
                    Color { r: 0.188, g: 0.835, b: 0.784, a: 0.2 });

                // player input to make a move on the board
                if is_mouse_button_released(MouseButton::Left) {
                    self.board.make_move(col_index);
                }
            }
            
            // AI turn if possible
            if self.board.turn == Cell::Red && self.board.state == BoardState::Active {
                match ai {
                   Beginner => ai::random::make_move(&mut self.board),
                   Easy => ai::expectiminimax::make_move(&mut self.board, 2, ai),
                   Medium => ai::expectiminimax::make_move(&mut self.board, 4, ai),
                   Hard => ai::expectiminimax::make_move(&mut self.board, 6, ai),
                   Impossible => todo!(),
                }
                
            }
            
            // render the result if the game is over
            if let Some(indices) = self.board.update_board_state() {
                let cell = if self.board.board[indices.0] == Cell::White { Cell::WhiteVictory } else { Cell::RedVictory };
                self.board.board[indices.0] = cell;
                self.board.board[indices.1] = cell;
                self.board.board[indices.2] = cell;
                self.board.board[indices.3] = cell;
            }

            draw_text(
                "Press 'r' to play again. Press 'q' to quit.", 
                offset, 
                screen_height() - d, 
                20.0, 
                WHITE);
        } else {
            let text = match self.board.state {
                BoardState::WhiteWon => "You won! Press 'r' to play again. Press 'q' to quit.",
                BoardState::RedWon => "The AI won! Press 'r' to play again. Press 'q' to quit.",
                BoardState::Draw => "Draw!\nPress 'r' to play again. Press 'q' to quit.",
                BoardState::Active => "GameState should not be 'Active' if the game is over. Contact admin."
            };

            draw_text(
                text, 
                offset, 
                screen_height() - d, 
                20.0, 
                WHITE);
        }

        // render the board
        for (i, cell) in self.board.iter().enumerate() {
            let x = (i % U_WIDTH) as f32;
            let y = (i / U_WIDTH) as f32;
            draw_circle(
                x*d + offset, 
                y*d + offset, 
                d/2.0, 
                cell.to_color()
            );

            draw_text(format!("{}", i).as_str(), x*d+offset, y*d+offset, 20., BLACK);
        }

        if is_key_pressed(KeyCode::R) {
            self.board.reset();
        } else if is_key_pressed(KeyCode::Q) {
            self.board.reset();
            target_scene = Menu;
        }

        target_scene
    }
}