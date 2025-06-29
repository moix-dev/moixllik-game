use crate::game::App;
use egui::Context;

pub fn show(app: &mut App, egui_ctx: &Context) {
    egui::Window::new("Acerca de").show(egui_ctx, |ui| {
        ui.heading("¿Qué es Moixllik?");
        ui.label(
            r#"
Es un proyecto que busca fomentar las culturas de los Andes, usándolas como inspiración.

El contenido de la biblioteca no es históricamente exacto.
"#,
        );
        ui.heading("¿Qué son los modos de juego?");
        ui.label(
            r#"
Los modos de juego son una reinterpretación del juego o herramienta andina yupana, usado para el cálculo matemático.

Los modos soportados:

- Modo Mapa: sirve para enriquecer las historias de la biblioteca.

- Modo Matemático: es una versión digital de la yupana.

- Modo Andes: es un juego de tablero enfocado en la estrategia.
"#
        );
        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.forms.remove("form-about");
            }
        });
    });
}
