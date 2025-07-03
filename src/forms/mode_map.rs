use crate::game::{App, Mode};
use crate::modes::map::{Map, get_faction_index, get_location, get_sector_index};
use egui::{Context, RichText};

#[derive(Default)]
struct Info {
    gloss: String,
    sector: String,
    faction: String,
    location: String,
}

pub fn show(app: &mut App, egui_ctx: &Context) {
    egui::Window::new("Modo Mapa").show(egui_ctx, |ui| {
        if app.mode == Mode::Map {
            app.title = app.mode_map.invader.track_message.clone();
            let info = get_info(app.row, app.column);
            ui.columns(2, |columns| {
                columns[0].horizontal_wrapped(|ui| {
                    ui.label("Sector:");
                });
                columns[1].label(info.sector);
                columns[0].horizontal_wrapped(|ui| {
                    ui.label("Facción:");
                });
                columns[1].label(info.faction);
                columns[0].horizontal_wrapped(|ui| {
                    ui.label("Ubicación:");
                });
                columns[1].label(info.location);
                columns[0].horizontal_wrapped(|ui| {
                    ui.label("Glosa:");
                });
            });
            ui.label(info.gloss);
            ui.separator();
            ui.collapsing("Configurar", |ui| {
                ui.checkbox(
                    &mut app.mode_map.enable_sector_lines,
                    "Dividir en sectores.",
                );
                ui.checkbox(
                    &mut app.mode_map.enable_faction_colors,
                    "Activar colores de facción.",
                );
            });
            ui.separator();
            ui.collapsing("Registro", |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Copiar").clicked() {
                        egui_ctx.copy_text(app.mode_map.log_text.clone());
                    }
                    if ui.button("Limpiar").clicked() {
                        app.mode_map.clear();
                    }
                    if ui.button("Ver").clicked() {
                        app.mode_map.parser();
                    }
                });
                ui.text_edit_multiline(&mut app.mode_map.log_text);
            });
        } else {
            if ui.button("¡Comenzar a jugar!").clicked() {
                app.mode = Mode::Map;
                app.mode_map.invader.random_move(vec![]);
                app.mode_map.invader.show_track(0, 0);
            }
            ui.separator();
            ui.label(
                RichText::new(
                    r#"- El juego consiste en buscar al invasor.
- Se darán pistas de los lugares que rodean al invasor.
- Al atrapar al invasor se moverá y ganas un punto."#,
                )
                .size(18.0),
            );
        }
        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.mode = Mode::None;
                app.mode_map = Map::default();
                app.forms.remove(&20);
            }
        });
    });
}

fn get_info(row: u8, column: u8) -> Info {
    let sector_index = get_sector_index(row, column);
    let faction_index = get_faction_index(row, column);
    Info {
        sector: get_sector(sector_index),
        faction: get_faction(faction_index),
        location: get_location(row, column),
        gloss: get_gloss(sector_index, faction_index),
    }
}

fn get_sector(index: usize) -> String {
    let sectors = ["X", "I", "II", "III", "IV"];
    sectors[index].to_string()
}

fn get_faction(index: usize) -> String {
    let factions = ["Rojo", "Azul", "Amarillo", "Verde", "Magenta", "Cian"];
    factions[index].to_string()
}

fn get_gloss(sector_index: usize, faction_index: usize) -> String {
    let sectors = [
        r#"En el sector Pacha se considera que existen tres planos:
1) Hanaq pacha, el plano de luz.
2) Kay pacha, el plano actual.
3) Uku pacha, el plano de la oscuridad.

Tienen como símbolo tradicional a dos serpientes que representan al sol y la luna."#,
        "En el sector Qolla tienen como símbolos tradicionales a la Llama y Zorro.",
        "En el sector Qonti tienen como símbolos tradicionales al Condor y Colibri.",
        "En el sector Chinchay tienen como símbolos tradicionales a la Iguana y Cangrejo.",
        "En el sector Anti tienen como símbolos tradicionales al Jaguar y Caimán.",
    ];
    let factions = [
        "En la facción de Rojo, son cercanos a los volcanes.",
        "En la facción de Azul, son cercanos a los mares.",
        "En la facción de Amarillo, son cercanos a los desiertos.",
        "En la facción de Verde, son cercanos a los bosques.",
        "En la facción de Magenta, son cercanos a las montañas.",
        "En la facción de Cian, son cercanos al cielo.",
    ];
    format!("{}\n\n{}", sectors[sector_index], factions[faction_index])
}
