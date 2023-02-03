use macroquad::prelude::*;
use std::cmp::min;

use crate::AIType::{self, *};
use crate::ui::Button;
use crate::{ai, board::*};

use super::scene_id::SceneId::{self, *};
use super::scene_trait::Scene;

#[derive(PartialEq)]
pub enum State {
    Active,
    Draw,
    RedWon,
    WhiteWon,
}

pub struct GameScene {
    board: Board,
    state: State,
    alpha_beta: ai::alpha_beta::AlphaBeta
}

impl GameScene {
    pub fn new() -> Self {
        GameScene {
            board: Board::new(),
            state: State::Active,
            alpha_beta: ai::alpha_beta::AlphaBeta::new()
        }
    }

    fn get_mouse_column(&self, pos: (f32, f32), offset: (f32, f32), d: f32) -> Option<usize> {
        let r = d/2.0;
        if pos.0 >= offset.0 && 
           pos.0 <= offset.0 + d*(F_WIDTH-1.0) + d &&
           pos.1 >= offset.1 - r &&
           pos.1 <= offset.1 + d*(F_HEIGHT-1.0) + d {
            let x = pos.0 - offset.0;
            Some(((x) / d) as usize)
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
            (screen_width() / (F_WIDTH + 4.)) as i32, 
            (screen_height() / (F_HEIGHT + 4.)) as i32
        ) as f32;

        let board_width = d * F_WIDTH;
        let board_height = d * F_WIDTH;
        let offset_width = (screen_width() - board_width) / 2.;
        let offset_height = (screen_height() - board_height) / 2.;

        if self.state == State::Active {
            let mouse_pos = mouse_position();
            let mouse_col = self.get_mouse_column(mouse_pos, (offset_width, offset_height), d);
            if let Some(col_index) = mouse_col {
                // highlight the column the player is hovering over
                draw_rectangle(
                    offset_width + d*col_index as f32, 
                    offset_height, 
                    d, 
                    d*F_HEIGHT, 
                    Color { r: 0.188, g: 0.835, b: 0.784, a: 0.2 });

                // player input to make a move on the board
                if is_mouse_button_released(MouseButton::Left) {
                    self.board.make_move(col_index);
                    // self.board.print_self();
                    if self.board.is_game_over(self.board.bit_board[0]) {
                        self.state = State::WhiteWon;
                    } else if self.board.is_draw() {
                        self.state = State::Draw;
                    }
                }
            }

            if self.state == State::Active && !self.board.is_white_turn() {
                // AI turn to make a move
                match ai {
                   Beginner => ai::random::make_move(&mut self.board),
                   Easy => self.alpha_beta.make_move(&mut self.board, 4, ai),
                   Medium => self.alpha_beta.make_move(&mut self.board, 10, ai),
                   Hard => self.alpha_beta.make_move(&mut self.board, 17, ai),
                   Impossible => self.alpha_beta.make_move(&mut self.board, 30, ai),
                }

                if self.board.is_game_over(self.board.bit_board[1]) {
                    self.state = State::RedWon;
                } else if self.board.is_draw() {
                    self.state = State::Draw;
                }
            }
        } else {
            let text = match self.state {
                State::WhiteWon => "You won!",
                State::RedWon => "AI won! ",
                State::Draw => "Draw!",
                State::Active => "GameState should not be 'Active' if the game is over. Contact admin."
            };

            draw_text(
                text, 
                screen_width()/2. - d, 
                d, 
                40.0, 
                WHITE);
        }

        // render the board
        for (i, cell) in self.board.get_cells().iter().enumerate() {
            let x = (i % S_WIDTH) as f32;
            let y = (i / S_WIDTH) as f32;
            draw_circle(
                x*d + d/2. + offset_width, 
                y*d + d/2. + offset_height, 
                d/2.0, 
                cell.to_color()
            );
        }

        // render buttons to restart or quit
        if Button::new()
            .pos(screen_width()/2. - d, screen_height() - screen_height()*0.1)
            .dimensions(78., 30.)
            .color(WHITE)
            .hover_color(BLUE)
            .text(" Restart".to_string())
            .font_size(20.)
            .font_color(BLACK)
            .draw() || is_key_pressed(KeyCode::R)
        {
            self.board.reset();
            self.state = State::Active;
        }
        else if Button::new()
            .pos(screen_width()/2. + d, screen_height() - screen_height()*0.1)
            .dimensions(50., 30.)
            .color(WHITE)
            .hover_color(BLUE)
            .text(" Quit".to_string())
            .font_size(20.)
            .font_color(BLACK)
            .draw() || is_key_pressed(KeyCode::Q)
        {
            self.board.reset();
            self.state = State::Active;
            target_scene = Menu;
        }

        target_scene
    }
}