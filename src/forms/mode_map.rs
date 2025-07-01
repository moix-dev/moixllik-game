use crate::game::{App, Mode};
use crate::modes::map::get_location;
use egui::Context;

#[derive(Default)]
struct Info {
    sector: String,
    faction: String,
    location: String,
    gloss: String,
}

pub fn show(app: &mut App, egui_ctx: &Context) {
    egui::Window::new("Moix Map").show(egui_ctx, |ui| {
        if app.mode == Mode::Map {
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
            ui.checkbox(
                &mut app.mode_map.enable_sector_lines,
                "Dividir en sectores.",
            );
            ui.checkbox(
                &mut app.mode_map.enable_faction_colors,
                "Usar color de la facción.",
            );
            ui.separator();
            ui.collapsing("Registro", |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Copiar").clicked() {
                        egui_ctx.copy_text(app.mode_map.log_text.clone());
                    }
                    if ui.button("Limpiar").clicked() {
                        app.mode_map.pieces.clear();
                        app.mode_map.log.clear();
                    }
                    if ui.button("Ver").clicked() {
                        app.mode_map.parser();
                    }
                });
                ui.text_edit_multiline(&mut app.mode_map.log_text);
            });
        } else {
            if ui.button("Iniciar modo de juego").clicked() {
                app.mode = Mode::Map;
                app.title = "Modo Map";
            }
            ui.label(format!("Modo actual: Moix {:?}", app.mode));
        }
        ui.separator();
        ui.vertical_centered(|ui| {
            if ui.button("Cerrar").clicked() {
                app.forms.remove("form-moix-map");
            }
        });
    });
}

fn get_info(row: f32, column: f32) -> Info {
    let (sector_index, sector) = get_sector(row, column);
    let (faction_index, faction) = get_faction(row, column);
    let location = get_location(row, column);
    let gloss = get_gloss(sector_index, faction_index);
    Info {
        sector,
        faction,
        location,
        gloss,
    }
}

fn get_sector(row: f32, column: f32) -> (usize, String) {
    let sectors = ["X", "I", "II", "III", "IV"];
    let index = match (row as usize, column as usize) {
        (r, c) if (3..7).contains(&r) && (4..7).contains(&c) => 1,
        (r, c) if (0..3).contains(&r) && (3..7).contains(&c) => 2,
        (r, c) if (0..4).contains(&r) && (0..3).contains(&c) => 3,
        (r, c) if (4..7).contains(&r) && (0..4).contains(&c) => 4,
        _ => 0,
    };
    (index, format!("{}", sectors[index]))
}

fn get_faction(row: f32, column: f32) -> (usize, String) {
    let factions = ["Rojo", "Azul", "Amarillo", "Verde", "Magenta", "Cian"];
    let index = match (row as usize, column as usize) {
        (3, 3) => 5,
        (r, c) if (2..5).contains(&r) && (2..5).contains(&c) => 4,
        (r, c) if (2..5).contains(&r) && (1..6).contains(&c) => 3,
        (r, c) if (1..6).contains(&r) && (2..5).contains(&c) => 3,
        (0, 3) | (3, 6) | (3, 0) | (6, 3) => 2,
        (0, 0) | (0, 6) | (6, 0) | (6, 6) => 0,
        _ => 1,
    };
    (index, format!("{}", factions[index]))
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
        "En la facción de Rojo, son cercanos a las islas o volcanes.",
        "En la facción de Azul, son cercanos a los mares.",
        "En la facción de Amarillo, son cercanos a los desiertos.",
        "En la facción de Verde, son cercanos a los bosques.",
        "En la facción de Magenta, son cercanos a las montañas.",
        "En la facción de Cian, son cercanos al cielo.",
    ];
    format!("{}\n\n{}", sectors[sector_index], factions[faction_index])
}
