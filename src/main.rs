mod board;
mod game;

use macroquad::prelude::*;

#[macroquad::main("Moixllik")]
async fn main() {
    let mut app = game::App::default();
    loop {
        clear_background(BROWN);
        let b = screen_width().min(screen_height()) / 9.0;
        let x = (screen_width() - b * 7.0) / 2.0;
        let y = (screen_height() - b * 7.0) / 2.0;

        board::draw(x, y, b);
        app.start(x, y, b);

        next_frame().await;
    }
}
