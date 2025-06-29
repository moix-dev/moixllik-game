use crate::forms;
use egui::{Context, RichText};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Llik,
    // Map,
    // Math,
    // Andes,
    // Streamer,
}

#[derive(Default)]
pub struct App<'a> {
    pub x: f32,
    pub y: f32,
    pub b: f32,
    pub focus: bool,
    pub mode: Mode,
    pub alert: &'a str,
    pub forms: HashMap<&'a str, bool>,
    pub user_session: Option<String>,
    pub config: Config,
}

#[derive(Default)]
pub struct Config {
    pub disable_board_title: bool,
    pub disable_marks_scales: bool,
}

impl App<'_> {
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
                        if self.user_session.is_none() {
                            if ui.button("Iniciar sesión").clicked() {
                                self.forms.entry("form-user-signin").or_insert(true);
                            }
                            if ui.button("Registrarse").clicked() {
                                self.forms.entry("form-user-signup").or_insert(true);
                            }
                        } else {
                            if ui.button("Mi cuenta").clicked() {
                                self.forms.entry("form-user").or_insert(true);
                            }
                            if ui.button("Cerrar sesión").clicked() {}
                        }
                        if ui.button("Eventos").clicked() {
                            self.forms.entry("form-user-events").or_insert(true);
                        }
                        if ui.button("Escuelas").clicked() {
                            self.forms.entry("form-user-schools").or_insert(true);
                        }
                        if ui.button("Rankings").clicked() {
                            self.forms.entry("form-user-rankings").or_insert(true);
                        }
                    });
                    ui.menu_button("Modo de juego", |ui| {
                        if ui.button("Moix Map").clicked() {
                            self.forms.entry("form-mode-map").or_insert(true);
                        }
                        if ui.button("Moix Math").clicked() {
                            self.forms.entry("form-mode-math").or_insert(true);
                        }
                        if ui.button("Moix Andes").clicked() {
                            self.forms.entry("form-mode-andes").or_insert(true);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        if ui.button("Moix Streamer").clicked() {
                            self.forms.entry("form-mode-streamer").or_insert(true);
                        }
                    });
                    #[cfg(target_arch = "wasm32")]
                    ui.menu_button("Descargar", |ui| {
                        ui.hyperlink_to(
                            RichText::new("Desde GitHub").color(color),
                            "https://github.com/moix-dev/moixllik-game/releases/tag/latest",
                        );
                    });

                    ui.menu_button("Ayuda", |ui| {
                        if ui.button("Acerca de").clicked() {
                            self.forms.entry("form-about").or_insert(true);
                        }
                        if ui.button("Configuración").clicked() {
                            self.forms.entry("form-config").or_insert(true);
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
                    ui.hyperlink_to("Biblioteca", "https://www.moixllik.com/library");
                    ui.separator();
                    ui.hyperlink_to("Tienda", "https://ko-fi.com/moixllik/shop");
                    ui.separator();
                    ui.hyperlink_to("Discord", "https://discord.gg/6me7JYRwS2");
                    ui.separator();
                    ui.label(egui::RichText::new(self.alert).color(egui::Color32::RED));
                });
            });
            self.show_forms(egui_ctx);
        });
        egui_macroquad::draw();
    }

    fn show_forms(&mut self, egui_ctx: &Context) {
        if self.forms.contains_key("form-about") {
            forms::about::show(self, egui_ctx);
        }
        if self.forms.contains_key("form-config") {
            forms::config::show(self, egui_ctx);
        }
    }
}
