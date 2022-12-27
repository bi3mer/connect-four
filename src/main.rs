use std::cmp::min;

use macroquad::prelude::*;
mod cell;

mod board;
use board::*;

mod game_scene;
mod menu_scene;

#[derive(PartialEq)]
enum Scene {
    Menu,
    Game
}

pub enum AI {
    Beginner,
    Easy,
    Medium,
    Hard,
    Impossible
}

#[macroquad::main("Connect-Four")]
async fn main() {
    let mut board = Board::new();
    let mut scene = Scene::Menu;
    let mut ai = AI::Medium;

    loop {
        clear_background(BLACK);

        let switch = match scene {
            Scene::Menu => menu_scene::update(&mut ai),
            Scene::Game => game_scene::update(&mut board),
        };

        if switch {
            scene = if scene == Scene::Menu { Scene::Game } else { Scene::Menu };
        }

        next_frame().await
    }
}