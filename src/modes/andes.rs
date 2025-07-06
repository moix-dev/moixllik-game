use crate::board::draw_piece_runa;
use macroquad::prelude::*;

#[derive(Default, Debug)]
pub struct Andes {}

impl Andes {
    pub fn set(&mut self) {}
    pub fn get(&mut self) {}
    pub fn clear(&mut self) {}
    pub fn draw(&mut self, x: f32, y: f32, b: f32) {
        // draw_piece_runa(3, 3, x, y, b, BLACK, 0, LIGHTGRAY);
        // draw_piece_runa(2, 3, x, y, b, BLACK, 1, LIGHTGRAY);
        // draw_piece_runa(4, 3, x, y, b, BLACK, 2, LIGHTGRAY);

        // draw_piece_runa(3, 3, x, y, b, WHITE, 0, DARKGRAY);
        // draw_piece_runa(2, 3, x, y, b, WHITE, 1, DARKGRAY);
        // draw_piece_runa(4, 3, x, y, b, WHITE, 2, DARKGRAY);
    }
}
