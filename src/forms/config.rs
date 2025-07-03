use crate::game::App;
use egui::{Context, Widget};

pub fn show(app: &mut App, egui_ctx: &Context) {
    egui::Window::new("Configuración").show(egui_ctx, |ui| {
        if ui.button("Restablecer aplicación").clicked() {
            *app = App::default();
        }
        ui.separator();
        ui.heading("Sonido");
        egui::Slider::new(&mut app.config.volume, 0.0..=1.0)
            .text("Volumen")
            .suffix("%")
            .step_by(0.01)
            .ui(ui);
        ui.checkbox(&mut app.config.disable_audio, "Desactivar audio.");
        ui.separator();
        ui.heading("Tablero");
        ui.checkbox(&mut app.config.enable_show_fps, "Mostrar FPS.");
        ui.checkbox(&mut app.config.disable_board_title, "Desactivar el título.");
        ui.checkbox(
            &mut app.config.enable_marks_scales,
            "Desactivar marcas y escalas.",
        );

        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.close_form(41);
            }
        });
    });
}
