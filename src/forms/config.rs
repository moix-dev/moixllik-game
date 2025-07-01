use crate::game::App;
use egui::Context;

pub fn show(app: &mut App, egui_ctx: &Context) {
    egui::Window::new("Configuración").show(egui_ctx, |ui| {
        if ui.button("Restablecer aplicación").clicked() {
            *app = App::default();
        }
        ui.separator();
        ui.checkbox(&mut app.config.disable_board_title, "Desactivar el título.");
        ui.checkbox(
            &mut app.config.enable_marks_scales,
            "Activar marcas y escalas.",
        );

        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.forms.remove("form-config");
            }
        });
    });
}
