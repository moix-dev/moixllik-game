use crate::forms;
use crate::modes;
use egui::{Context, RichText};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug, Eq, PartialEq)]
pub enum Mode {
    #[default]
    None,
    Map,
    // Math,
    // Andes,
    // Streamer,
}

#[derive(Default)]
pub struct App {
    pub x: f32,
    pub y: f32,
    pub b: f32,
    pub row: u8,
    pub column: u8,
    pub focus: bool,
    pub title: String,
    // Forms
    pub config: Config,
    pub alert: String,
    pub user_session: String,
    pub forms: HashMap<u8, bool>,
    // Modes
    pub mode: Mode,
    pub mode_map: modes::map::Map,
}

#[derive(Default)]
pub struct Config {
    pub enable_show_fps: bool,
    pub disable_board_title: bool,
    pub enable_marks_scales: bool,
}

impl App {
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
                                self.forms.entry(10).or_insert(true);
                            }
                            if ui.button("Registrarse").clicked() {
                                self.forms.entry(11).or_insert(true);
                            }
                        } else {
                            if ui.button("Mi cuenta").clicked() {
                                self.forms.entry(12).or_insert(true);
                            }
                            if ui.button("Cerrar sesión").clicked() {}
                        }
                        ui.separator();
                        if ui.button("Eventos").clicked() {
                            self.forms.entry(13).or_insert(true);
                        }
                        if ui.button("Escuelas").clicked() {
                            self.forms.entry(14).or_insert(true);
                        }
                        if ui.button("Rankings").clicked() {
                            self.forms.entry(15).or_insert(true);
                        }
                    });
                    ui.menu_button("Modo de juego", |ui| {
                        // Id: 2X
                        if ui.button("Modo Mapa").clicked() {
                            self.forms.entry(20).or_insert(true);
                        }
                        if ui.button("Mode Matemático").clicked() {
                            self.forms.entry(21).or_insert(true);
                        }
                        if ui.button("Modo Andes").clicked() {
                            self.forms.entry(22).or_insert(true);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        if ui.button("Modo Streamer").clicked() {
                            self.forms.entry(23).or_insert(true);
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
                            self.forms.entry(40).or_insert(true);
                        }
                        if ui.button("Configuración").clicked() {
                            self.forms.entry(41).or_insert(true);
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
                    ui.label(egui::RichText::new(self.alert.as_str()).color(egui::Color32::RED));
                });
            });
            self.show_forms(egui_ctx);
        });
        egui_macroquad::draw();
    }

    fn show_forms(&mut self, egui_ctx: &Context) {
        // 2X: Modos de juego
        if self.forms.contains_key(&20) {
            forms::mode_map::show(self, egui_ctx);
        }
        // 4X: Ayuda
        if self.forms.contains_key(&40) {
            forms::about::show(self, egui_ctx);
        }
        if self.forms.contains_key(&41) {
            forms::config::show(self, egui_ctx);
        }
    }
}
