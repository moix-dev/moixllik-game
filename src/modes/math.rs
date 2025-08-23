use crate::board::draw_piece_small;
use macroquad::prelude::*;

#[derive(Default, Debug)]
pub struct Math {
    pub sun_title: String,
    pub moon_title: String,
}

impl Math {
    pub fn set(&mut self) {}
    pub fn get(&mut self) {}
    pub fn clear(&mut self) {}
    pub fn draw(&mut self, x: f32, y: f32, b: f32) {
        draw_piece_small(3, 3, x, y, b, BLACK);
        draw_piece_small(3, 3, x, y + b * 0.5, b, BLACK);

        draw_piece_small(3, 3, x + b * 0.5, y, b, WHITE);
        draw_piece_small(3, 3, x + b * 0.5, y + b * 0.5, b, WHITE);
    }
}
