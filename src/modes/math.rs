use crate::board::draw_piece_small;
use macroquad::prelude::*;

#[derive(Default, Debug)]
pub struct Math {
    pub sun_value: u32,
    pub sun_title: String,
    pub moon_value: u32,
    pub moon_title: String,
}

impl Math {
    pub fn set(&mut self) {}
    pub fn get(&mut self) {}
    pub fn clear(&mut self) {
        *self = Math::default();
    }
    pub fn draw(&mut self, x: f32, y: f32, b: f32) {
        // self.sun_value = 1_000;
        // self.moon_value = 1_000_000;

        self.sun_title = to_title(&self.sun_value);
        self.moon_title = to_title(&self.moon_value);

        // draw_piece_small(3, 3, x, y, b, BLACK);
        // draw_piece_small(3, 3, x, y + b * 0.5, b, BLACK);

        // draw_piece_small(3, 3, x + b * 0.5, y, b, WHITE);
        // draw_piece_small(3, 3, x + b * 0.5, y + b * 0.5, b, WHITE);
    }
}

pub fn to_title(n: &u32) -> String {
    let mut s = n.to_string();
    let mut result  = String::new();

    while s.len() > 3 {
        let split = s.split_off(s.len() - 3);
        result = format!(" {}{}", split, result);
    }
    result = format!("{}{}", s, result);

    result
}