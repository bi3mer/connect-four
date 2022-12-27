use macroquad::{prelude::*, ui::{widgets, root_ui, hash}};

pub fn update(ai: &mut super::AI) -> bool {

    root_ui().label(None, "hello megaui");
    if root_ui().button(vec2(300.0, 300.0), "Beginner") {
        *ai = super::AI::Beginner;
    }

    if root_ui().button(vec2(380.0, 300.0), "Easy") {
        *ai = super::AI::Easy;
    }

    if root_ui().button(vec2(420.0, 300.0), "Medium") {
        *ai = super::AI::Medium;
    }

    false
}