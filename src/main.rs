use macroquad::prelude::*;

#[macroquad::main("Moixllik")]
async fn main() {
    loop {
        clear_background(BROWN);
        let b = screen_width().min(screen_height()) / 9.0;
        let x = (screen_width() - b * 7.0) / 2.0;
        let y = (screen_height() - b * 7.0) / 2.0;
        draw_rectangle(x - b * 0.10, y - b * 0.10, b * 7.20, b * 7.20, DARKBROWN);
        draw_rectangle(x, y, b * 7.0, b * 7.0, ORANGE);

        for i in 1..7 {
            let i = i as f32;
            draw_line(x, y + b * i, x + b * 7.0, y + b * i, 1.0, GRAY);
            draw_line(x + b * i, y, x + b * i, y + b * 7.0, 1.0, GRAY);
        }

        let color = Color::from_rgba(0, 0, 0, 30);
        draw_rectangle(x, y, b, b, color);
        draw_rectangle(x, y + b * 6.0, b, b, color);
        draw_rectangle(x + b * 6.0, y, b, b, color);
        draw_rectangle(x + b * 6.0, y + b * 6.0, b, b, color);

        draw_rectangle(x + b * 3.0, y, b * 1.0, b * 7.0, color);
        draw_rectangle(x, y + b * 3.0, b * 7.0, b * 1.0, color);

        draw_rectangle(x + b * 2.0, y + b, b * 3.0, b * 5.0, color);
        draw_rectangle(x + b, y + b * 2.0, b * 5.0, b * 3.0, color);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("MOIXLLIK").show(egui_ctx, |ui| {
                ui.label("En construcción");
            });
        });
        egui_macroquad::draw();

        next_frame().await;
    }
}
