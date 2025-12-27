use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Cursor Crisis - Rust Edition".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}
