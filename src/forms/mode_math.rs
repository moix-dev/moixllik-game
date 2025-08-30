use crate::game::App;
use egui::{Context, RichText};

pub fn show(app: &mut App, egui_ctx: &Context) {
    egui::Window::new("Modo Matemático").show(egui_ctx, |ui| {
        if app.mode == 2 {
            if ui.button("Finalizar modo de juego").clicked() {
                app.mode = 0;
                app.mode_math.clear();
            }
            ui.separator();
            ui.checkbox(
                &mut app.mode_math.enable_values,
                "Mostrar el valor de las marcas.",
            );
        } else {
            if ui.button("Iniciar modo de juego").clicked() {
                app.mode = 2;
            }
        }
        ui.separator();
        ui.label(RichText::new(r#"Se calcula en tiempo real."#).size(18.0));
        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.close_form(21);
            }
        });
    });
}
