use crate::ui;
use macroquad::{prelude::{WHITE, RED, GRAY, vec2, BLUE}, window::{screen_width, screen_height}, text::draw_text};

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
    if ui::button(
        vec2(w/2.3, h/2.),
        vec2(80., 50.),
        GRAY,
        RED,
        " Play",
        32.,
        WHITE,
        true) {
        change_scene = true;
    } 

    if ui::button(
        vec2(button_x, button_height),
        vec2(70., 30.),
        if *ai == crate::AIType::Beginner { RED } else { GRAY },
        BLUE,
        " Beginner",
        15.,
        WHITE,
        *ai != crate::AIType::Beginner) {
        *ai = crate::AIType::Beginner;
    }

    if ui::button(
        vec2(button_x+90., button_height),
        vec2(40., 30.),
        if *ai == crate::AIType::Easy { RED } else { GRAY },
        BLUE,
        " Easy",
        15.,
        WHITE,
        *ai != crate::AIType::Easy) {
        *ai = crate::AIType::Easy;
    }

    if ui::button(
        vec2(button_x+150., button_height),
        vec2(60., 30.),
        if *ai == crate::AIType::Medium { RED } else { GRAY },
        BLUE,
        " Medium",
        15.,
        WHITE,
        *ai != crate::AIType::Medium) {
        *ai = crate::AIType::Medium;
    }

    if ui::button(
        vec2(button_x+230., button_height),
        vec2(40., 30.),
        if *ai == crate::AIType::Hard { RED } else { GRAY },
        BLUE,
        " Hard",
        15.,
        WHITE,
        *ai != crate::AIType::Hard) {
        *ai = crate::AIType::Hard;
    }

    if ui::button(
        vec2(button_x+290., button_height),
        vec2(80., 30.),
        if *ai == crate::AIType::Impossible { RED } else { GRAY },
        BLUE,
        " Impossible",
        15.,
        WHITE,
        *ai != crate::AIType::Impossible) {
        *ai = crate::AIType::Impossible;
    }
    
    change_scene
}