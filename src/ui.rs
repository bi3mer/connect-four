use macroquad::prelude::*;

pub struct Button {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
    hover_color: Option<Color>,
    text: Option<String>,
    font_size: f32,
    font_color: Color,
    is_active: bool,
    clicked: bool
}

impl Button {
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,
            color: RED,
            hover_color: None,
            text: None,
            font_size: 12.,
            font_color: BLACK,
            is_active: true,
            clicked: false
        }
    }

    pub fn pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn dimensions(&mut self, w: f32, h: f32) -> &mut Self {
        self.w = w;
        self.h = h;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn hover_color(&mut self, color: Color) -> &mut Self {
        self.hover_color = Some(color);
        self
    } 

    pub fn text(&mut self, text: String) -> &mut Self {
        self.text = Some(text);
        self
    }

    pub fn font_size(&mut self, size: f32) -> &mut Self {
        self.font_size = size;
        self
    }

    pub fn font_color(&mut self, color: Color) -> &mut Self {
        self.font_color = color;
        self
    }

    pub fn is_active(&mut self, active: bool) -> &mut Self {
        self.is_active = active;
        self
    }

    pub fn draw(&mut self) -> bool {
        let p = mouse_position();
        let mouse_in_bounds = 
            self.is_active && 
            p.0 >= self.x && 
            p.0 <= self.x+self.w && 
            p.1 >= self.y && 
            p.1 <= self.y+self.h;

        let rec_color = 
            if self.hover_color.is_some() && mouse_in_bounds 
                { self.hover_color.unwrap() } 
            else 
                { self.color };

        draw_rectangle(
            self.x,
            self.y,
            self.w,
            self.h,
            rec_color
        );

        if self.text.is_some() {
            draw_text(
                self.text.as_ref().unwrap().as_str(),
                self.x,
                self.y + self.h/2.,
                self.font_size,
                self.font_color
            ); 
        }

        self.clicked = 
            self.is_active && 
            mouse_in_bounds && 
            is_mouse_button_released(MouseButton::Left);

        self.clicked
    }
}
