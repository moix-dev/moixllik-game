use crate::board;
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Map {
    pub enable_sector_lines: bool,
    pub enable_faction_colors: bool,
    pub pieces: HashMap<String, Piece>,
    pub log: Vec<String>,
    pub log_text: String,
}

#[derive(Default, Clone)]
pub struct Piece {
    row: f32,
    column: f32,
}

impl Map {
    pub fn draw(&mut self, x: f32, y: f32, b: f32) {
        for (_key, piece) in self.pieces.iter() {
            let color = if self.enable_faction_colors {
                get_faction_color(piece.row, piece.column)
            } else {
                WHITE
            };
            board::draw_piece_big(piece.row, piece.column, x, y, b, color);
        }
    }
    pub fn pressed(&mut self, row: f32, column: f32) {
        let log_len = self.log.len() + 1;
        let key = get_location(row, column);

        if self.pieces.contains_key(&key) {
            self.action_remove(log_len, key);
        } else {
            self.action_add(log_len, key, row, column);
        }

        // LOG
        self.log_text = self.log.join("; ");
    }
    fn action_add(&mut self, log_len: usize, key: String, row: f32, column: f32) {
        self.pieces.insert(key.clone(), Piece { row, column });
        self.log.push(format!("{}.{}", log_len, key));
    }
    fn action_remove(&mut self, log_len: usize, key: String) {
        self.pieces.remove(&key);
        self.log.push(format!("{}.{}+", log_len, key));
    }
    pub fn parser(&mut self) {
        let log_text = self.log_text.clone();
        let mut steps = log_text.split("; ");
        while let Some(step) = steps.next() {
            let parts = step.splitn(2, '.').collect::<Vec<_>>();
            if parts.len() == 2 {
                let log_len = parts[0].parse::<usize>().unwrap_or(0);
                let data = parts[1];
                let mark = data.chars().nth(0).unwrap();
                let scale = data.chars().nth(1).unwrap();
                let key = format!("{}{}", mark, scale);
                let row = 6.0 - "abcdefg".find(mark).unwrap_or(0) as f32;
                let column = 6.0 - scale.to_digit(10).unwrap_or(0) as f32;
                match data.chars().nth(2).unwrap_or(' ') {
                    '+' => self.action_remove(log_len, key),
                    _ => self.action_add(log_len, key, row, column),
                }
            }
        }
    }
}

pub fn get_location(row: f32, column: f32) -> String {
    let marks = ["g", "f", "e", "d", "c", "b", "a"];
    let mark = marks[row as usize];
    let scale = (6.0 - column) as u32;
    format!("{}{}", mark, scale)
}

fn get_faction_color(row: f32, column: f32) -> Color {
    let factions = [
        Color::from_hex(0xFF0000),
        Color::from_hex(0x0000FF),
        Color::from_hex(0xFFFF00),
        Color::from_hex(0x00FF00),
        Color::from_hex(0xFF00FF),
        Color::from_hex(0x00FFFF),
    ];
    let index = match (row as usize, column as usize) {
        (3, 3) => 5,
        (r, c) if (2..5).contains(&r) && (2..5).contains(&c) => 4,
        (r, c) if (2..5).contains(&r) && (1..6).contains(&c) => 3,
        (r, c) if (1..6).contains(&r) && (2..5).contains(&c) => 3,
        (0, 3) | (3, 6) | (3, 0) | (6, 3) => 2,
        (0, 0) | (0, 6) | (6, 0) | (6, 6) => 0,
        _ => 1,
    };
    factions[index]
}
