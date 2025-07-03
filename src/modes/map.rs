use crate::board;
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Map {
    pub invader: Invader,
    pub enable_sector_lines: bool,
    pub enable_faction_colors: bool,
    pub pieces: HashMap<u8, Piece>,
    // LOG
    pub log: Vec<String>,
    pub log_text: String,
}

#[derive(Default, Debug)]
pub struct Invader {
    pub row: u8,
    pub column: u8,
    pub points: u16,
    // Track
    pub track_row: i8,
    pub track_column: i8,
    pub track_count: u16,
    pub track_message: String,
}

#[derive(Default, Debug, Clone)]
pub struct Piece {
    row: u8,
    column: u8,
}

impl Map {
    pub fn clear(&mut self) {
        self.log.clear();
        self.pieces.clear();
        self.invader = Invader::default();
    }
    pub fn draw(&mut self, x: f32, y: f32, b: f32) {
        for (_key, piece) in self.pieces.iter() {
            let color = if self.enable_faction_colors {
                get_faction_color(piece.row, piece.column)
            } else {
                WHITE
            };
            board::draw_piece_big(piece.row as f32, piece.column as f32, x, y, b, color);
        }
    }
    pub fn pressed(&mut self, row: u8, column: u8) {
        let log_len = self.log.len() + 1;
        let key = row * column;
        if self.pieces.contains_key(&key) {
            self.action_remove(log_len, key, row, column);
        } else {
            self.action_add(log_len, key, row, column);
            if self.invader.row == row && self.invader.column == column {
                let pieces = self.pieces.keys().collect::<Vec<&u8>>();
                self.invader.points += 1;
                self.invader.random_move(pieces);
            }
        }
        self.invader.show_track(log_len, self.pieces.len());

        // LOG
        self.log_text = self.log.join("; ");
    }
    fn action_add(&mut self, log_len: usize, key: u8, row: u8, column: u8) {
        self.pieces.insert(key, Piece { row, column });
        let location = get_location(row, column);
        self.log.push(format!("{}.{}", log_len, location));
    }
    fn action_remove(&mut self, log_len: usize, key: u8, row: u8, column: u8) {
        self.pieces.remove(&key);
        let location = get_location(row, column);
        self.log.push(format!("{}.{}+", log_len, location));
    }
    pub fn parser(&mut self) {
        let log_text = self.log_text.clone();
        self.clear();
        let mut steps = log_text.split("; ");
        while let Some(step) = steps.next() {
            let parts = step.splitn(2, '.').collect::<Vec<_>>();
            if parts.len() == 2 {
                let log_len = parts[0].parse::<usize>().unwrap_or(0);
                let data = parts[1];
                let mark = data.chars().nth(0).unwrap();
                let scale = data.chars().nth(1).unwrap();
                let row = 6 - "abcdefg".find(mark).unwrap_or(0) as u8;
                let column = 6 - scale.to_digit(10).unwrap_or(0) as u8;
                let key = row * column;
                match data.chars().nth(2).unwrap_or(' ') {
                    '+' => self.action_remove(log_len, key, row, column),
                    _ => self.action_add(log_len, key, row, column),
                }
            }
        }
    }
}

impl Invader {
    pub fn random_move(&mut self, pieces: Vec<&u8>) {
        rand::srand(miniquad::date::now() as u64);
        loop {
            if pieces.len() == 49 {
                return;
            }
            let row: u8 = rand::gen_range(0, 7);
            let column: u8 = rand::gen_range(0, 7);
            let key = row * column;
            if !pieces.iter().any(|&x| x == &key) {
                self.row = row;
                self.column = column;
                return;
            }
        }
    }
    pub fn show_track(&mut self, turn: usize, pieces_len: usize) {
        rand::srand(miniquad::date::now() as u64);
        if pieces_len == 49 {
            self.track_message = format!("Puntaje: {} / {}", self.points, turn);
        } else {
            let vals = [-1, 1];
            let mut row = vals[rand::gen_range(0, 2)];
            let mut column = vals[rand::gen_range(0, 2)];
            if row == self.track_row && column == self.track_column {
                if row == self.track_row {
                    row *= -1;
                } else if column == self.track_column {
                    column *= -1;
                }
            }
            self.track_row = row;
            self.track_column = column;
            let sector = get_sector_symbol(self.row, self.column);
            let faction = get_faction_symbol(
                (self.row as i8 + self.track_row).clamp(0, 6) as u8,
                (self.column as i8 + self.track_column).clamp(0, 6) as u8,
            );
            self.track_count += 1;
            self.track_message = format!(
                "Puntaje: {} / {} | Pista {}: {}, {}",
                self.points, turn, self.track_count, sector, faction,
            );
        }
    }
}

pub fn get_location(row: u8, column: u8) -> String {
    let scale = 6 - column;
    let mark = "gfedcba".chars().nth(row as usize).unwrap();
    format!("{}{}", mark, scale)
}

pub fn get_sector_index(row: u8, column: u8) -> usize {
    match (row, column) {
        (r, c) if (3..7).contains(&r) && (4..7).contains(&c) => 1,
        (r, c) if (0..3).contains(&r) && (3..7).contains(&c) => 2,
        (r, c) if (0..4).contains(&r) && (0..3).contains(&c) => 3,
        (r, c) if (4..7).contains(&r) && (0..4).contains(&c) => 4,
        _ => 0,
    }
}

pub fn get_faction_index(row: u8, column: u8) -> usize {
    match (row, column) {
        (3, 3) => 5,
        (r, c) if (2..5).contains(&r) && (2..5).contains(&c) => 4,
        (r, c) if (2..5).contains(&r) && (1..6).contains(&c) => 3,
        (r, c) if (1..6).contains(&r) && (2..5).contains(&c) => 3,
        (0, 3) | (3, 6) | (3, 0) | (6, 3) => 2,
        (0, 0) | (0, 6) | (6, 0) | (6, 6) => 0,
        _ => 1,
    }
}

fn get_faction_color(row: u8, column: u8) -> Color {
    let factions = [
        Color::from_hex(0xFF0000),
        Color::from_hex(0x0000FF),
        Color::from_hex(0xFFFF00),
        Color::from_hex(0x00FF00),
        Color::from_hex(0xFF00FF),
        Color::from_hex(0x00FFFF),
    ];
    let index = get_faction_index(row, column);
    factions[index]
}

fn get_sector_symbol(row: u8, column: u8) -> &'static str {
    let sectors = [
        ["Pacha", "Sol", "Luna"],
        ["Qolla", "Llama", "Zorro"],
        ["Qonti", "Condor", "Colibri"],
        ["Chinchay", "Iguana", "Cangrejo"],
        ["Anti", "Jaguar", "Caimán"],
    ];
    let index = get_sector_index(row, column);
    let sector = sectors[index];
    sector[rand::gen_range(0, sector.len())]
}

fn get_faction_symbol(row: u8, column: u8) -> &'static str {
    let factions = [
        ["Rojo", "Isla", "Volcán"],
        ["Azul", "Océano", "Mar"],
        ["Amarillo", "Desierto", "Arena"],
        ["Verde", "Selva", "Bosque"],
        ["Magenta", "Montaña", "Nieve"],
        ["Cian", "Estrellas", "Cielo"],
    ];
    let index = get_faction_index(row, column);
    let faction = factions[index];
    faction[rand::gen_range(0, faction.len())]
}
