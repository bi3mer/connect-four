use macroquad::prelude::*;
mod cell;
mod ui;

mod board;
use board::*;

mod game_scene;
mod menu_scene;

mod ai;

#[derive(PartialEq)]
enum Scene {
    Menu,
    Game
}

#[derive(PartialEq)]
pub enum AIType {
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
    let mut ai = AIType::Easy;

    loop {
        clear_background(BLACK);

        let switch = match scene {
            Scene::Menu => menu_scene::update(&mut ai),
            Scene::Game => game_scene::update(&mut board, &ai),
        };

        if switch {
            scene = if scene == Scene::Menu { Scene::Game } else { Scene::Menu };
        }

        next_frame().await
    }
}