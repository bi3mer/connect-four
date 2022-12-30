use crate::ui;
use macroquad::{prelude::{WHITE, RED, GRAY, BLUE}, window::{screen_width, screen_height}, text::draw_text};

pub fn update(ai: &mut super::AIType) -> bool {
    let w = screen_width();
    let h = screen_height();

    let button_x = w / 4.3;
    let button_height = h - (h/3.);

    draw_text(
        "Connect-Four", 
        screen_width()/3.7, 
        screen_height()/3., 
        60., 
        WHITE
    );

    let mut change_scene = false;

    if ui::Button::new()
        .pos(w/2.3, h/2.)
        .dimensions(80., 50.)
        .color(GRAY)
        .hover_color(WHITE)
        .text(" Play".to_string())
        .font_size(32.)
        .font_color(WHITE)
        .is_active(true)
        .draw()
    {
        change_scene = true;
    }

    if ui::Button::new()
        .pos(button_x, button_height)
        .dimensions(70., 30.)
        .color(if *ai == crate::AIType::Beginner { RED } else { GRAY })
        .hover_color(BLUE)
        .text(" Beginner".to_string())
        .font_size(15.)
        .font_color(WHITE)
        .is_active(*ai != crate::AIType::Beginner)
        .draw() 
    {
        *ai = crate::AIType::Beginner;
    }

    if ui::Button::new()
        .pos(button_x + 90., button_height)
        .dimensions(40., 30.)
        .color(if *ai == crate::AIType::Easy { RED } else { GRAY })
        .hover_color(BLUE)
        .text(" Easy".to_string())
        .font_size(15.)
        .font_color(WHITE)
        .is_active(*ai != crate::AIType::Easy)
        .draw() 
    {
        *ai = crate::AIType::Easy;
    }

    if ui::Button::new()
        .pos(button_x + 150., button_height)
        .dimensions(60., 30.)
        .color(if *ai == crate::AIType::Medium { RED } else { GRAY })
        .hover_color(BLUE)
        .text(" Medium".to_string())
        .font_size(15.)
        .font_color(WHITE)
        .is_active(*ai != crate::AIType::Medium)
        .draw() 
    {
        *ai = crate::AIType::Medium;
    }

    if ui::Button::new()
        .pos(button_x + 230., button_height)
        .dimensions(40., 30.)
        .color(if *ai == crate::AIType::Hard { RED } else { GRAY })
        .hover_color(BLUE)
        .text(" Hard".to_string())
        .font_size(15.)
        .font_color(WHITE)
        .is_active(*ai != crate::AIType::Hard)
        .draw() 
    {
        *ai = crate::AIType::Hard;
    }

    if ui::Button::new()
        .pos(button_x + 290., button_height)
        .dimensions(80., 30.)
        .color(if *ai == crate::AIType::Impossible { RED } else { GRAY })
        .hover_color(BLUE)
        .text(" Impossible".to_string())
        .font_size(15.)
        .font_color(WHITE)
        .is_active(*ai != crate::AIType::Impossible)
        .draw() 
    {
        *ai = crate::AIType::Impossible;
    }
    
    change_scene
}