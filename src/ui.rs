use macroquad::prelude::*;

pub fn button(
    pos: Vec2, 
    bounds: Vec2, 
    color: Color, 
    hover_color: Color, 
    text: &str, 
    font_size: f32, 
    font_color: Color,
    active: bool
) -> bool {
    let p = mouse_position();
    let mouse_in_bounds = active && p.0 >= pos.x && p.0 <= pos.x+bounds.x && p.1 >= pos.y && p.1 <= pos.y+bounds.y;

    draw_rectangle(
        pos.x,
        pos.y,
        bounds.x,
        bounds.y,
        if mouse_in_bounds { hover_color } else { color }
    );

    draw_text(
        text,
        pos.x,
        pos.y + bounds.y/2.,
        font_size,
        font_color
    ); 

    active && mouse_in_bounds && is_mouse_button_pressed(MouseButton::Left)
}
