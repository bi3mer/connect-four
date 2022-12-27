use crate::ui;
use macroquad::{prelude::{WHITE, RED, GRAY, vec2, BLUE}, window::{screen_width, screen_height}, text::draw_text};

pub fn update(ai: &mut super::AI) -> bool {
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
        if *ai == crate::AI::Beginner { RED } else { GRAY },
        BLUE,
        " Beginner",
        15.,
        WHITE,
        *ai != crate::AI::Beginner) {
        *ai = crate::AI::Beginner;
    }

    if ui::button(
        vec2(button_x+90., button_height),
        vec2(40., 30.),
        if *ai == crate::AI::Easy { RED } else { GRAY },
        BLUE,
        " Easy",
        15.,
        WHITE,
        *ai != crate::AI::Easy) {
        *ai = crate::AI::Easy;
    }

    if ui::button(
        vec2(button_x+150., button_height),
        vec2(60., 30.),
        if *ai == crate::AI::Medium { RED } else { GRAY },
        BLUE,
        " Medium",
        15.,
        WHITE,
        *ai != crate::AI::Medium) {
        *ai = crate::AI::Medium;
    }

    if ui::button(
        vec2(button_x+230., button_height),
        vec2(40., 30.),
        if *ai == crate::AI::Hard { RED } else { GRAY },
        BLUE,
        " Hard",
        15.,
        WHITE,
        *ai != crate::AI::Hard) {
        *ai = crate::AI::Hard;
    }

    if ui::button(
        vec2(button_x+290., button_height),
        vec2(80., 30.),
        if *ai == crate::AI::Impossible { RED } else { GRAY },
        BLUE,
        " Impossible",
        15.,
        WHITE,
        *ai != crate::AI::Impossible) {
        *ai = crate::AI::Impossible;
    }
    
    change_scene
}