use crate::game::App;
use macroquad::prelude::*;

pub fn show_fps(x: f32, y: f32, b: f32) {
    let fps = format!("FPS: {}", get_fps());
    draw_text(fps.as_str(), x + b * 6.0, y + b * 7.5, b * 0.3, WHITE);
}

pub fn draw(x: f32, y: f32, b: f32) {
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
}

pub fn pointer(app: &mut App, x: f32, y: f32, b: f32) {
    let (mx, my) = mouse_position();
    if (mx >= x && mx <= x + b * 7.0) && (my >= y && my <= y + b * 7.0) {
        let color = Color::from_rgba(0, 0, 0, 100);
        let xx = ((mx - x) / b).trunc();
        let yy = ((my - y) / b).trunc();

        draw_rectangle(xx * b + x, yy * b + y, b, b, color);

        app.row = xx as u8;
        app.column = yy as u8;

        if is_mouse_button_released(MouseButton::Left) {
            match app.mode {
                1 => match app.mode_map.pressed(xx as u8, yy as u8) {
                    1 => app.sound.ply = true,
                    2 => app.sound.badge = true,
                    _ => (),
                },
                _ => (),
            }
        }
    }
}

pub fn draw_title(x: f32, y: f32, b: f32, text: &str) {
    draw_text(text, x, y - b * 0.35, b * 0.3, WHITE);
}

pub fn draw_marks_scales(x: f32, y: f32, b: f32) {
    let color = LIGHTGRAY;
    let font_size = b * 0.2;
    let marks = ["a", "b", "c", "d", "e", "f", "g"];
    for scale in 0..7 {
        let i = scale as f32;
        draw_text(
            marks[6 - scale],
            x + b * (i + 0.5),
            y + b * 7.25,
            font_size,
            color,
        );
        draw_text(
            (6 - scale).to_string().as_str(),
            x + b * 7.15,
            y + b * (i + 0.5),
            font_size,
            color,
        );
        draw_text(
            marks[scale],
            x + b * (i + 0.5),
            y - b * 0.17,
            font_size,
            color,
        );
        draw_text(
            (scale).to_string().as_str(),
            x - b * 0.25,
            y + b * (i + 0.5),
            font_size,
            color,
        );
    }
}

pub fn draw_sector_lines(x: f32, y: f32, b: f32) {
    let color = DARKBROWN;
    let line = b * 0.05;
    draw_line(
        x + b * 3.0,
        y + b * 4.0,
        x + b * 7.0,
        y + b * 4.0,
        line,
        color,
    );
    draw_line(
        x + b * 3.0,
        y + b * 3.0,
        x + b * 3.0,
        y + b * 7.0,
        line,
        color,
    );
    draw_line(x, y + b * 3.0, x + b * 4.0, y + b * 3.0, line, color);
    draw_line(x + b * 4.0, y, x + b * 4.0, y + b * 4.0, line, color);
}

pub fn draw_piece_big(row: u8, column: u8, x: f32, y: f32, b: f32, color: Color) {
    let x = x + b * (row as f32 + 0.5);
    let y = y + b * (column as f32 + 0.5);
    draw_circle(x, y, b * 0.42, DARKBROWN);
    draw_circle(x, y, b * 0.4, color);
}
