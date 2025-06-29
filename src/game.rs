use macroquad::prelude::*;

#[derive(Default)]
pub enum Mode {
    #[default]
    Map,
}

#[derive(Default)]
pub struct App {
    pub x: f32,
    pub y: f32,
    pub b: f32,
    pub mode: Mode,
    pub alert: String,
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
            egui::TopBottomPanel::top("panel-top").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("MOIXLLIK");
                    ui.separator();
                    ui.menu_button("Jugador", |ui| {
                        ui.label("En construcción");
                    });
                    ui.menu_button("Modo de juego", |ui| {
                        ui.label("En construcción");
                    });
                    ui.menu_button("Ayuda", |ui| {
                        ui.label("En construcción");
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
                    ui.label(egui::RichText::new(&self.alert).color(egui::Color32::RED));
                });
            });
        });
        egui_macroquad::draw();
    }
}
