use macroquad::prelude::*;
mod cell;
mod ui;

mod board;

mod scene;
use scene::{scene_trait::Scene, menu_scene::MenuScene};

use crate::scene::scene_id::SceneId::*;

mod ai;


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
    let mut scene = Menu;
    let mut ai = AIType::Easy;
    
    // let mut board = Board::new();
    let mut menu_scene = MenuScene::new();
    let mut current_scene: &mut dyn Scene = &mut menu_scene;

    loop {
        clear_background(BLACK);

        let new_scene = current_scene.update(&mut ai);
        if new_scene != scene {
            println!("switch!");
            if scene == Game {
                scene = Menu;
                // current_scene = &menu_scene as &mut dyn Scene;
            } else {
                scene = Menu;
                println!("not implemented!");
            }
        }

        next_frame().await
    }
}