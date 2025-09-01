use crate::board::draw_piece_big;
use macroquad::{prelude::*, rand::ChooseRandom};

#[derive(Default, Debug)]
pub struct Map {
    pub board: u64, // uX - 1
    pub invader: Invader,
    pub enable_sector_lines: bool,
    pub enable_faction_colors: bool,
    // LOG
    pub log: Vec<[u8; 3]>,
    pub log_text: String,
}

#[derive(Default, Debug)]
pub struct Invader {
    pub row: u8,
    pub column: u8,
    pub points: u16,
    pub message: String,
}

impl Map {
    fn set(&mut self, row: u8, column: u8, value: bool) {
        let mask = row * 7 + column;
        if value {
            self.board |= 1 << mask;
        } else {
            self.board &= !(1 << mask);
        }
    }
    fn get(&mut self, row: u8, column: u8) -> bool {
        let mask = row * 7 + column;
        self.board & (1 << mask) != 0
    }
    pub fn clear(&mut self) {
        self.log.clear();
        self.board = 0;
        self.invader = Invader::default();
    }
    pub fn draw(&mut self, x: f32, y: f32, b: f32) {
        for i in 0..49 {
            let row = i / 7;
            let column = i % 7;
            if self.get(row, column) {
                let color = if self.enable_faction_colors {
                    get_faction_color(row, column)
                } else {
                    WHITE
                };
                draw_piece_big(row, column, x, y, b, color);
            }
        }
    }
    #[allow(unused_assignments)]
    pub fn pressed(&mut self, row: u8, column: u8) -> u8 {
        let mut event = 0;
        if self.get(row, column) {
            self.action_remove(row, column);
            event = 3;
        } else {
            self.action_add(row, column);
            // Point UP!
            if self.invader.row == row && self.invader.column == column {
                self.invader.points += 1;
                self.invader.random_move(&self.board);
                event = 2;
            } else {
                event = 1;
            }
        }
        self.invader.show_track(self.log.len(), &self.board);
        self.parser_log();

        event
    }
    fn action_add(&mut self, row: u8, column: u8) {
        self.set(row, column, true);
        self.log.push([row, column, 0]);
    }
    fn action_remove(&mut self, row: u8, column: u8) {
        self.set(row, column, false);
        self.log.push([row, column, 1]);
    }
    pub fn parser(&mut self) {
        let log_text = self.log_text.clone();
        self.clear();
        let mut steps = log_text.split("; ");
        while let Some(step) = steps.next() {
            let parts = step.splitn(2, '.').collect::<Vec<_>>();
            if parts.len() == 2 {
                let data = parts[1];
                let mark = data.chars().nth(0).unwrap();
                let scale = data.chars().nth(1).unwrap();
                let row = 6 - "abcdefg".find(mark).unwrap_or(0) as u8;
                let column = 6 - scale.to_digit(10).unwrap_or(0) as u8;
                match data.chars().nth(2).unwrap_or(' ') {
                    '+' => self.action_remove(row, column),
                    _ => self.action_add(row, column),
                }
            }
        }
    }
    pub fn parser_log(&mut self) {
        self.log_text = self
            .log
            .iter()
            .enumerate()
            .map(|(index, step)| {
                format!(
                    "{}.{}{}",
                    index + 1,
                    get_location(step[0], step[1]),
                    if step[2] == 1 { "+" } else { "" }
                )
            })
            .collect::<Vec<String>>()
            .join("; ");
    }
}

impl Invader {
    pub fn random_move(&mut self, board: &u64) {
        if board == &((1u64 << 49) - 1) {
            return;
        }
        let mut slots: Vec<(u8, u8)> = vec![];
        for x in 0..49 {
            if (board & (1 << x)) == 0 {
                let row = x / 7;
                let column = x % 7;
                slots.push((row, column));
            }
        }

        rand::srand(miniquad::date::now() as u64);
        if let Some((row, column)) = slots.choose() {
            self.row = *row;
            self.column = *column;
        }
    }
    pub fn show_track(&mut self, log_len: usize, board: &u64) {
        if board == &((1u64 << 49) - 1) {
            self.message = format!("Puntaje: {} / {}", self.points, log_len);
        } else {
            rand::srand(miniquad::date::now() as u64);
            let row = (self.row as i8 + rand::gen_range(0, 3) - 1).clamp(0, 6);
            let column = (self.column as i8 + rand::gen_range(0, 3) - 1).clamp(0, 6);
            let sector = get_sector_symbol(self.row, self.column);
            let faction = get_faction_symbol(row as u8, column as u8);
            self.message = format!(
                "Puntaje: {} / {} | Pista: {}, {}.",
                self.points, log_len, sector, faction
            );
        }
    }
}

pub fn get_location(row: u8, column: u8) -> String {
    let scale = 6 - column;
    let mark = "gfedcba".chars().nth(row as usize).unwrap();
    format!("{}{}", mark, scale)
}

pub fn get_sector_index(row: u8, column: u8) -> u8 {
    match (row, column) {
        (r, c) if (3..7).contains(&r) && (4..7).contains(&c) => 1,
        (r, c) if (0..3).contains(&r) && (3..7).contains(&c) => 2,
        (r, c) if (0..4).contains(&r) && (0..3).contains(&c) => 3,
        (r, c) if (4..7).contains(&r) && (0..4).contains(&c) => 4,
        _ => 0,
    }
}

pub fn get_faction_index(row: u8, column: u8) -> u8 {
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
    factions[index as usize]
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
    let sector = sectors[index as usize];
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
    let faction = factions[index as usize];
    faction[rand::gen_range(0, faction.len())]
}
