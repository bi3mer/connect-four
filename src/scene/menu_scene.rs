use macroquad::{prelude::{WHITE, RED, GRAY, BLUE}, window::{screen_width, screen_height}, text::{draw_text, get_text_center}};
use crate::scene::scene_trait::Scene;
use crate::AIType::{self, *};
use crate::ui::Button;
use super::scene_id::SceneId::{self, *};

pub struct MenuScene {
    play_button: Button,
    beginner_button: Button,
    easy_button: Button,
    medium_button: Button,
    hard_button: Button,
    impossible_button: Button,
}

impl MenuScene {
    pub fn new() -> Self {
        let mut play_button = Button::new();
        play_button
            .dimensions(80., 50.)
            .color(GRAY)
            .hover_color(WHITE)
            .text(" Play".to_string())
            .font_size(32.)
            .font_color(WHITE)
            .is_active(true);

        let mut beginner_button = Button::new();
        beginner_button
            .dimensions(70., 30.)
            .hover_color(BLUE)
            .text(" Beginner".to_string())
            .font_size(15.)
            .font_color(WHITE);

        let mut easy_button = Button::new();
        easy_button
            .dimensions(40., 30.)
            .hover_color(BLUE)
            .text(" Easy".to_string())
            .font_size(15.)
            .font_color(WHITE);

        let mut medium_button = Button::new();
        medium_button
            .dimensions(60., 30.)
            .hover_color(BLUE)
            .text(" Medium".to_string())
            .font_size(15.)
            .font_color(WHITE);

        let mut hard_button = Button::new();
        hard_button
            .dimensions(40., 30.)
            .hover_color(BLUE)
            .text(" Hard".to_string())
            .font_size(15.)
            .font_color(WHITE);

        let mut impossible_button = Button::new();
        impossible_button
            .dimensions(80., 30.)
            .hover_color(BLUE)
            .text(" Impossible".to_string())
            .font_size(15.)
            .font_color(WHITE);
            
        MenuScene {
            play_button,
            beginner_button,
            easy_button,
            medium_button,
            hard_button,
            impossible_button
        }
    }
}

impl Scene for MenuScene {
    fn update(&mut self, ai: &mut AIType) -> SceneId {
        let w = screen_width();
        let h = screen_height();

        let button_x = w / 2.;
        let button_height = h - (h/3.5);

        // draw title
        let mut center = get_text_center("Connect-Four", None, 60, 1., 0.);
        draw_text(
            "Connect-Four", 
            screen_width()/2. - center.x, 
            screen_height()/3., 
            60., 
            WHITE
        );

        // draw buttons
        let mut target_scene = Menu;
        center = get_text_center(" Play", None, 32, 1., 0.);
        if self.play_button.pos(w/2. - center.x, h/2. - center.y).draw() {
            target_scene = Game;
        }

        if self.beginner_button
            .pos(button_x-220., button_height)
            .color(if *ai == Beginner { RED } else { GRAY })
            .is_active(*ai != Beginner)
            .draw()
        {
            *ai = Beginner;
        }

        if self.easy_button
            .pos(button_x-125., button_height)
            .color(if *ai == Easy { RED } else { GRAY })
            .is_active(*ai != Easy)
            .draw()
        {
            *ai = Easy;
        }

        if self.medium_button
            .pos(button_x - 60., button_height)
            .color(if *ai == Medium { RED } else { GRAY })
            .is_active(*ai != Medium)
            .draw() 
        {
            *ai = Medium;
        }

        if self.hard_button
            .pos(button_x + 25., button_height)
            .color(if *ai == Hard { RED } else { GRAY })
            .is_active(*ai != Hard)
            .draw() 
        {
            *ai = Hard;
        }

        if !cfg!(target_arch = "wasm32") {
            if self.impossible_button
                .pos(button_x + 100., button_height)
                .color(if *ai == Impossible { RED } else { GRAY })
                .is_active(*ai != Impossible)
                .draw() 
            {
                *ai = Impossible;
            }
        } else {
            self.impossible_button
                .pos(button_x + 100., button_height)
                .color(GRAY)
                .is_active(false)
                .draw(); 

            let text = "Impossible AI disabled for web version.";
            center = get_text_center(text, None, 15, 1., 0.);
            draw_text(
                text, 
                screen_width()/2. - center.x, 
                screen_height() - screen_height()*0.1, 
                15.,
                WHITE
            );
        }

        target_scene
    }
}
