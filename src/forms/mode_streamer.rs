use crate::game::App;
use egui::{Context, RichText};

pub fn show(app: &mut App, egui_ctx: &Context) {
    egui::Window::new("Modo Streamer").show(egui_ctx, |ui| {
        if app.mode == 4 {
            if ui.button("Finalizar modo de juego").clicked() {
                app.mode = 0;
            }
            ui.separator();
        } else {
            if ui.button("¡Comenzar a jugar!").clicked() {
                app.mode = 4;
            }
            ui.separator();
            ui.label(RichText::new(r#""#).size(18.0));
        }
        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.close_form(23);
            }
        });
    });
}
