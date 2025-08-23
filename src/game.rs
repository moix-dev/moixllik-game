use crate::forms;
use crate::modes;
use egui::{Context, RichText};
use macroquad::prelude::*;

#[derive(Default)]
pub struct App {
    pub x: f32,
    pub y: f32,
    pub b: f32,
    pub row: u8,
    pub column: u8,
    pub focus: bool,
    // Sound
    pub sound: Sound,
    // Forms
    pub config: Config,
    pub alert: String,
    pub user_session: String,
    pub forms: u64, // max_forms = uX - 1
    // Modes
    pub mode: u8,
    pub mode_map: modes::map::Map,
    pub mode_math: modes::math::Math,
    pub mode_andes: modes::andes::Andes,
    #[cfg(not(target_arch = "wasm32"))]
    pub mode_streamer: modes::streamer::Streamer,
}

// #[derive(Default)]
pub struct Config {
    pub volume: f32,
    pub disable_audio: bool,
    pub enable_show_fps: bool,
    pub disable_board_title: bool,
    pub enable_marks_scales: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            volume: 0.5,
            disable_audio: false,
            enable_show_fps: false,
            disable_board_title: false,
            enable_marks_scales: false,
        }
    }
}

#[derive(Default)]
pub struct Sound {
    pub ply: bool,
    pub badge: bool,
}

impl App {
    pub fn open_form(&mut self, id: u8) {
        self.forms |= 1 << id;
    }
    pub fn close_form(&mut self, id: u8) {
        self.forms &= !(1 << id);
    }
    // pub fn toggle_form(&mut self, id: u8) {
    //     self.forms ^= 1 << id;
    // }
    pub fn is_open_form(&mut self, id: u8) -> bool {
        (self.forms & (1 << id)) != 0
    }
    pub fn start(&mut self, x: f32, y: f32, b: f32) {
        self.x = x;
        self.y = y;
        self.b = b;
        self.manager();
    }
    fn manager(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            self.focus = egui_ctx.is_pointer_over_area();
            let color = egui::Color32::GRAY;
            egui::TopBottomPanel::top("panel-top").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("MOIXLLIK");
                    ui.separator();
                    ui.menu_button("Jugador", |ui| {
                        // Id: 1X
                        if self.user_session.len() == 0 {
                            if ui.button("Iniciar sesión").clicked() {
                                self.open_form(10);
                            }
                            if ui.button("Registrarse").clicked() {
                                self.open_form(11);
                            }
                        } else {
                            if ui.button("Mi cuenta").clicked() {
                                self.open_form(12);
                            }
                            if ui.button("Cerrar sesión").clicked() {}
                        }
                        ui.separator();
                        if ui.button("Eventos").clicked() {
                            self.open_form(13);
                        }
                        if ui.button("Escuelas").clicked() {
                            self.open_form(14);
                        }
                        if ui.button("Ranking").clicked() {
                            self.open_form(15);
                        }
                    });
                    ui.menu_button("Modo de juego", |ui| {
                        // Id: 2X
                        if ui.button("Modo Mapa").clicked() {
                            self.open_form(20);
                        }
                        if ui.button("Mode Matemático").clicked() {
                            self.open_form(21);
                        }
                        if ui.button("Modo Andes").clicked() {
                            self.open_form(22);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        if ui.button("Modo Streamer").clicked() {
                            self.open_form(23);
                        }
                    });
                    #[cfg(target_arch = "wasm32")]
                    ui.menu_button("Descargar", |ui| {
                        // Id: 3X
                        ui.hyperlink_to(
                            RichText::new("Desde GitHub").color(color),
                            "https://github.com/moix-dev/moixllik-game/releases/tag/latest",
                        );
                    });

                    ui.menu_button("Ayuda", |ui| {
                        // Id: 4X
                        if ui.button("Acerca de").clicked() {
                            self.open_form(40);
                        }
                        if ui.button("Configuración").clicked() {
                            self.open_form(41);
                        }
                        ui.hyperlink_to(
                            RichText::new("Reportar errores").color(color),
                            "https://github.com/moix-dev/moixllik-game/issues",
                        );
                    });
                });
            });
            egui::TopBottomPanel::bottom("panel-bottom").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.hyperlink_to("Web", "https://www.moixllik.com/");
                    ui.separator();
                    ui.hyperlink_to("Biblioteca", "https://www.moixllik.com/corpus");
                    ui.separator();
                    ui.hyperlink_to("Tienda", "https://ko-fi.com/moixllik/shop");
                    ui.separator();
                    ui.hyperlink_to("Discord", "https://discord.gg/6me7JYRwS2");
                    ui.separator();
                    ui.hyperlink_to("Developer", "https://www.moix.cc/");
                    ui.separator();
                    ui.label(egui::RichText::new(&self.alert).color(egui::Color32::RED));
                });
            });
            self.show_forms(egui_ctx);
        });
        egui_macroquad::draw();
    }

    fn show_forms(&mut self, egui_ctx: &Context) {
        // 2X: Modos de juego
        if self.is_open_form(20) {
            forms::mode_map::show(self, egui_ctx);
        }
        if self.is_open_form(21) {
            forms::mode_math::show(self, egui_ctx);
        }
        if self.is_open_form(22) {
            forms::mode_andes::show(self, egui_ctx);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.is_open_form(23) {
            forms::mode_streamer::show(self, egui_ctx);
        }
        // 4X: Ayuda
        if self.is_open_form(40) {
            forms::about::show(self, egui_ctx);
        }
        if self.is_open_form(41) {
            forms::config::show(self, egui_ctx);
        }
    }
}
