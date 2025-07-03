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

- Moix Map, es el modo mapa que sirve para enriquecer las historias de la biblioteca.

- Moix Math, es el modo matemático funciona como una versión digital de la yupana.

- Moix Andes, es un modo de juego en el tablero enfocado en la estrategia.
"#
        );
        #[cfg(not(target_arch = "wasm32"))]
        ui.label("- Moix Streamer, es un modo que desbloquea la interacción con el chat de Twitch.");
        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.close_form(40);
            }
        });
    });
}
