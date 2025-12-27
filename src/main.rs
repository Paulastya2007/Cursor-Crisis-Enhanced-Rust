// Hide console window on Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod config;
mod game;
mod popup;
pub mod sound_gen;
mod ui;

use assets::GameAssets;
use config::window_conf;
use game::GameState;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let assets = GameAssets::load().await;
    let mut game = GameState::new();

    show_mouse(false);

    loop {
        let dt = get_frame_time();

        game.update(dt, &assets);
        game.draw(&assets);

        next_frame().await;
    }
}
