use crate::game::App;
use egui::Context;

pub fn show(app: &mut App, egui_ctx: &Context) {
    egui::Window::new("Configuración").show(egui_ctx, |ui| {
        ui.checkbox(&mut app.config.disable_board_title, "Desactivar el título.");
        ui.checkbox(
            &mut app.config.disable_marks_scales,
            "Desactivar marcas y escalas.",
        );

        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.forms.remove("form-config");
            }
        });
    });
}
