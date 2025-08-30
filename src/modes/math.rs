use crate::board::draw_piece_small;
use macroquad::prelude::*;

#[derive(Default, Debug)]
pub struct Math {
    pub board: [u64; 4],
    pub sun_value: u32,
    pub sun_title: String,
    pub moon_value: u32,
    pub moon_title: String,
    pub enable_values: bool,
}

impl Math {
    fn set(&mut self, row: u8, column: u8, value: bool) {
        let mask = row as usize * 14 + column as usize;
        let idx = mask / 64;
        let shift = mask % 64;
        if value {
            self.board[idx] |= 1 << shift;
        } else {
            self.board[idx] &= !(1 << shift);
        }
    }
    fn get(&mut self, row: u8, column: u8) -> bool {
        let mask = row as usize * 14 + column as usize;
        let idx = mask / 64;
        let shift = mask % 64;
        self.board[idx] & (1 << shift) != 0
    }
    pub fn clear(&mut self) {
        *self = Math::default();
    }
    pub fn draw(&mut self, x: f32, y: f32, b: f32) {
        self.sun_title = to_title(&self.sun_value);
        self.moon_title = to_title(&self.moon_value);

        for i in 0..196 {
            let row = i / 14;
            let column = i % 14;
            if self.get(row, column) {
                let color = if row % 2 == 0 { BLACK } else { WHITE };
                draw_piece_small(row, column, x, y, b, color);
            }
        }
    }

    #[allow(unused_assignments)]
    pub fn pressed(&mut self, row: u8, column: u8) -> u8 {
        let mut event = 0;

        if self.get(row, column) {
            self.action_remove(row, column);
            event = 2;
        } else {
            self.action_add(row, column);
            event = 1;
        }
        self.calculate();

        event
    }

    fn action_add(&mut self, row: u8, column: u8) {
        self.set(row, column, true);
    }
    fn action_remove(&mut self, row: u8, column: u8) {
        self.set(row, column, false);
    }

    fn calculate(&mut self) {
        let marks = [1, 2, 3, 5, 3, 2, 1];
        let mut sun_value = 0;
        let mut moon_value = 0;

        for i in 0..196 {
            let row = i / 14;
            let column = i % 14;
            if self.get(row, column) {
                let r = 6 - row / 2;
                let c = 6 - column / 2;
                let value = marks[r as usize] * 10_u32.pow(c as u32);

                if row % 2 == 0 {
                    moon_value += value;
                } else {
                    sun_value += value;
                }
            }
        }

        self.sun_value = sun_value;
        self.moon_value = moon_value;
    }
}

pub fn to_title(n: &u32) -> String {
    let mut s = n.to_string();
    let mut result = String::new();

    while s.len() > 3 {
        let split = s.split_off(s.len() - 3);
        result = format!(" {}{}", split, result);
    }
    result = format!("{}{}", s, result);

    result
}
