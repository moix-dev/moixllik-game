use crate::game::App;
use egui::{Context, Layout, RichText, ScrollArea, Slider};

pub fn show(app: &mut App, egui_ctx: &Context) {
    egui::Window::new("Modo Matemático").show(egui_ctx, |ui| {
        if app.mode == 2 {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Finalizar modo de juego").clicked() {
                    app.mode = 0;
                    app.mode_math.clear();
                }
                if ui.button("Limpiar tablero").clicked() {
                    app.mode_math.clear_board();
                }
            });
            ui.separator();
            ui.collapsing("Configuración", |ui| {
                ui.checkbox(
                    &mut app.mode_math.enable_values,
                    "Mostrar el valor de las marcas.",
                );
            });
            ui.separator();
            ui.collapsing("Historial", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.add(Slider::new(&mut app.mode_math.scale_random, 0..=6).text("Escala"));
                    if ui.button("Generar").clicked() {
                        app.mode_math.generate_random();
                    }
                });
                // ui.text_edit_multiline(&mut app.mode_math.log_text);
                ScrollArea::vertical().max_height(60.0).show(ui, |ui| {
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(&app.mode_math.log_text)
                                .monospace()
                                .size(24.0)
                                .strong(),
                        );
                    });
                });
            });
        } else {
            if ui.button("Iniciar modo de juego").clicked() {
                app.mode = 2;
                app.mode_math.scale_random = 3;
            }
            ui.separator();
            ui.label(
                RichText::new(
                    r#"Realiza operaciones matemáticas con total libertad en el tablero."#,
                )
                .size(18.0),
            );
        }
        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.close_form(21);
            }
        });
    });
}
