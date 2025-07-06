mod board;
mod forms;
mod game;
mod modes;
mod sound;

use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Moixllik".to_string(),
        icon: Some(Icon {
            // convert icon_x.png -depth 8 RGBA:icon_x.raw
            small: include_bytes!("../assets/icon_16.raw").clone(),
            medium: include_bytes!("../assets/icon_32.raw").clone(),
            big: include_bytes!("../assets/icon_64.raw").clone(),
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = game::App::default();
    let sound_ply = sound::Sound::new(include_bytes!("../assets/sound/ply.ogg"));
    let sound_badge = sound::Sound::new(include_bytes!("../assets/sound/badge.ogg"));
    loop {
        // Audio
        if !app.config.disable_audio {
            if app.sound.ply {
                sound_ply.play(app.config.volume, false);
                app.sound.ply = false;
            }
            if app.sound.badge {
                sound_badge.play(app.config.volume, false);
                app.sound.badge = false;
            }
        }

        // View
        clear_background(BROWN);

        let b = screen_width().min(screen_height()) / 9.0;
        let x = (screen_width() - b * 7.0) / 2.0;
        let y = (screen_height() - b * 7.0) / 2.0;

        board::draw(x, y, b);

        // Config
        if app.config.enable_show_fps {
            board::show_fps(x, y, b);
        }
        if !app.config.enable_marks_scales {
            board::draw_marks_scales(x, y, b);
        }
        // Modes
        if !app.focus {
            board::pointer(&mut app, x, y, b);
        }
        match app.mode {
            1 => {
                if !app.config.disable_board_title {
                    board::draw_title(x, y, b, app.mode_map.invader.message.as_str(), WHITE);
                }
                if app.mode_map.enable_sector_lines {
                    board::draw_sector_lines(x, y, b);
                }
                app.mode_map.draw(x, y, b);
            }
            2 => {
                app.mode_math.draw(x, y, b);
            }
            3 => {
                app.mode_andes.draw(x, y, b);
            }
            #[cfg(not(target_arch = "wasm32"))]
            4 => {
                app.mode_streamer.draw(x, y, b);
            }
            _ => (),
        }
        // Forms
        app.start(x, y, b);

        next_frame().await;
    }
}
