use crate::assets::GameAssets;
use crate::game::{VIRTUAL_H, VIRTUAL_W};
use macroquad::prelude::*;

pub struct UI;

impl UI {
    pub fn draw_bar(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        value: f32,
        label: &str,
        color: Color,
        assets: &GameAssets,
        scale: f32,
    ) {
        // Draw background
        draw_texture_ex(
            &assets.bar_bg,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );

        // Draw fill
        let bar_texture = if color == RED {
            &assets.bar_red
        } else {
            &assets.bar_yellow
        };
        draw_texture_ex(
            bar_texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w * value, h)),
                source: Some(Rect::new(
                    0.0,
                    0.0,
                    bar_texture.width() * value,
                    bar_texture.height(),
                )),
                ..Default::default()
            },
        );

        // Draw label
        draw_text_ex(
            label,
            x + 10.0 * scale,
            y + h + 15.0 * scale,
            TextParams {
                font: Some(&assets.font),
                font_size: (14.0 * scale) as u16,
                color,
                ..Default::default()
            },
        );
    }

    pub fn draw_score_and_popups(
        score: u32,
        popup_count: usize,
        scale: f32,
        offset_x: f32,
        offset_y: f32,
        assets: &GameAssets,
    ) {
        draw_text_ex(
            &format!("Score: {}", score),
            offset_x + 20.0 * scale,
            offset_y + 30.0 * scale,
            TextParams {
                font: Some(&assets.font),
                font_size: (24.0 * scale) as u16,
                color: WHITE,
                ..Default::default()
            },
        );

        draw_text_ex(
            &format!("Popups: {}", popup_count),
            offset_x + 20.0 * scale,
            offset_y + 55.0 * scale,
            TextParams {
                font: Some(&assets.font),
                font_size: (20.0 * scale) as u16,
                color: GRAY,
                ..Default::default()
            },
        );
    }

    pub fn draw_game_over(scale: f32, offset_x: f32, offset_y: f32, assets: &GameAssets) {
        let go_txt = "GAME OVER";
        let font_size = (60.0 * scale) as u16;
        let text_size = measure_text(go_txt, Some(&assets.font), font_size, 1.0);

        draw_text_ex(
            go_txt,
            offset_x + (VIRTUAL_W * scale - text_size.width) / 2.0,
            offset_y + (VIRTUAL_H * scale) / 2.0,
            TextParams {
                font: Some(&assets.font),
                font_size,
                color: RED,
                ..Default::default()
            },
        );

        let restart_txt = "PRESS 'R' TO RESTART";
        let restart_size = (24.0 * scale) as u16;
        let restart_text_size = measure_text(restart_txt, Some(&assets.font), restart_size, 1.0);

        draw_text_ex(
            restart_txt,
            offset_x + (VIRTUAL_W * scale - restart_text_size.width) / 2.0,
            offset_y + (VIRTUAL_H * scale) / 2.0 + 70.0 * scale,
            TextParams {
                font: Some(&assets.font),
                font_size: restart_size,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}
