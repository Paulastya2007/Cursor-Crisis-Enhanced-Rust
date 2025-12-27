use crate::assets::GameAssets;
use crate::popup::Popup;
use crate::ui::UI;
use macroquad::audio::{PlaySoundParams, play_sound};
use macroquad::prelude::*;
use macroquad_particles::{ColorCurve, Emitter, EmitterConfig};
pub const VIRTUAL_W: f32 = 800.0;
pub const VIRTUAL_H: f32 = 600.0;
pub const SPAWN_INTERVAL: f32 = 1.2;
pub const EXPLOSION_RADIUS: f32 = 90.0;
pub const ENERGY_COST: f32 = 0.2;
pub const ENERGY_REGEN: f32 = 0.1;
pub const DAMAGE_RATE: f32 = 0.15;

pub struct ExplosionCircle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub timer: f32,
}
// ---------------- PARTICLE CONFIG ----------------

fn particle_explosion() -> EmitterConfig {
    EmitterConfig {
        amount: 25,
        lifetime: 0.4,
        emitting: true,
        initial_direction_spread: 360.0,
        initial_velocity: 180.0,
        initial_velocity_randomness: 0.7,
        size: 6.0,
        size_randomness: 0.6,
        gravity: vec2(0.0, 120.0),
        ..Default::default()
    }
}

pub struct GameState {
    pub popups: Vec<Popup>,
    pub explosions: Vec<ExplosionCircle>,
    pub emitters: Vec<(Emitter, Vec2)>,
    pub spawn_timer: f32,
    pub score: u32,
    pub health: f32,
    pub energy: f32,
    pub game_over_sound_played: bool,
    pub start_sound_played: bool,
    pub frames_since_start: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            popups: Vec::new(),
            explosions: Vec::new(),
            emitters: Vec::new(),
            spawn_timer: 0.0,
            score: 0,
            health: 1.0,
            energy: 1.0,
            game_over_sound_played: false,
            start_sound_played: false,
            frames_since_start: 0,
        }
    }

    pub fn reset(&mut self) {
        self.health = 1.0;
        self.energy = 1.0;
        self.score = 0;
        self.game_over_sound_played = false;
        self.popups.clear();
        self.explosions.clear();
        self.emitters.clear();
    }

    pub fn update(&mut self, dt: f32, assets: &GameAssets) {
        self.frames_since_start += 1;

        if !self.start_sound_played && self.frames_since_start > 10 {
            play_sound(
                &assets.snd_start,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
            self.start_sound_played = true;
        }

        if self.health <= 0.0 {
            if is_key_pressed(KeyCode::R) {
                self.reset();
                play_sound(
                    &assets.snd_start,
                    PlaySoundParams {
                        looped: false,
                        volume: 1.0,
                    },
                );
            }
            return;
        }

        // Energy regeneration
        self.energy = (self.energy + ENERGY_REGEN * dt).min(1.0);

        // Get scaling
        let (scale, offset_x, offset_y) = self.get_scaling();
        let (raw_mx, raw_my) = mouse_position();
        let mx = (raw_mx - offset_x) / scale;
        let my = (raw_my - offset_y) / scale;

        // Spawn system
        self.spawn_timer += dt;
        if self.spawn_timer >= SPAWN_INTERVAL {
            self.popups
                .push(Popup::new(VIRTUAL_W, VIRTUAL_H, assets.char_bodies.len()));
            self.spawn_timer = 0.0;
        }

        // Update popups & Health drain
        for popup in self.popups.iter_mut() {
            popup.update(dt);
            popup.follow(mx, my, dt);
            if popup.hit(mx, my) {
                self.health -= DAMAGE_RATE * dt;
            }
        }
        self.health = self.health.max(0.0);

        // Explosion logic
        let mut exploded = is_mouse_button_pressed(MouseButton::Right);
        if exploded && self.energy < ENERGY_COST {
            exploded = false;
        }
        let mut explosion_requests: Vec<Vec2> = Vec::new();

        self.explosions.retain_mut(|e| {
            e.timer += dt;
            e.timer < 0.2
        });

        // Emitters will be updated and filtered in draw phase

        if exploded {
            self.energy -= ENERGY_COST;
            let current_score = &mut self.score;
            self.popups.retain(|p: &Popup| {
                let closest_x = mx.clamp(p.x, p.x + p.w);
                let closest_y = my.clamp(p.y, p.y + p.h);
                let dx = mx - closest_x;
                let dy = my - closest_y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq <= EXPLOSION_RADIUS * EXPLOSION_RADIUS {
                    *current_score += 1;
                    explosion_requests.push(vec2(p.x + p.w / 2.0, p.y + p.h / 2.0));

                    false
                } else {
                    true
                }
            });

            play_sound(
                &assets.snd_click,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );

            self.explosions.push(ExplosionCircle {
                x: raw_mx,
                y: raw_my,
                radius: EXPLOSION_RADIUS * scale,
                timer: 0.0,
            });

            // Spawn particle emitters at explosion request positions
            for pos in explosion_requests.iter() {
                let mut config = particle_explosion();
                config.colors_curve = ColorCurve {
                    start: Color::new(1.0, 0.647, 0.0, 1.0),
                    mid: Color::new(1.0, 0.4, 0.0, 0.8),
                    end: Color::new(0.5, 0.0, 0.0, 0.0),
                };
                let mut emitter = Emitter::new(config);
                emitter.emit(*pos, 25);
                self.emitters.push((emitter, *pos));
            }
        }

        if self.health <= 0.0 && !self.game_over_sound_played {
            play_sound(
                &assets.snd_over,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
            self.game_over_sound_played = true;
        }
    }

    pub fn draw(&mut self, assets: &GameAssets) {
        let (scale, offset_x, offset_y) = self.get_scaling();
        let (raw_mx, raw_my) = mouse_position();

        clear_background(BLACK);

        // Draw popups
        for popup in self.popups.iter() {
            popup.draw_scaled(scale, offset_x, offset_y, &assets.char_bodies);
        }

        // Draw explosions
        for e in self.explosions.iter() {
            let alpha = 1.0 - (e.timer / 0.2);
            draw_circle_lines(e.x, e.y, e.radius, 3.0, Color::new(1.0, 0.647, 0.0, alpha));
        }

        // Draw particle emitters
        for (emitter, pos) in self.emitters.iter_mut() {
            emitter.draw(*pos);
        }

        // Draw UI
        self.render_ui(scale, offset_x, offset_y, assets);

        // Draw Game Over if needed
        if self.health <= 0.0 {
            UI::draw_game_over(scale, offset_x, offset_y, assets);
        }

        // Custom cursor
        let cursor_w = 22.0 * scale;
        let cursor_h = 28.0 * scale;
        draw_texture_ex(
            &assets.cursor_texture,
            raw_mx,
            raw_my,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(cursor_w, cursor_h)),
                ..Default::default()
            },
        );
    }

    fn get_scaling(&self) -> (f32, f32, f32) {
        let scale_x = screen_width() / VIRTUAL_W;
        let scale_y = screen_height() / VIRTUAL_H;
        let scale = scale_x.min(scale_y);

        let offset_x = (screen_width() - VIRTUAL_W * scale) / 2.0;
        let offset_y = (screen_height() - VIRTUAL_H * scale) / 2.0;
        (scale, offset_x, offset_y)
    }

    fn render_ui(&self, scale: f32, offset_x: f32, offset_y: f32, assets: &GameAssets) {
        let bar_w = 140.0 * scale;
        let bar_h = 24.0 * scale;
        let ui_x = offset_x + VIRTUAL_W * scale - bar_w - 20.0 * scale;
        let ui_y = offset_y + 20.0 * scale;

        // Health Bar
        UI::draw_bar(
            ui_x,
            ui_y,
            bar_w,
            bar_h,
            self.health,
            "HEALTH",
            RED,
            assets,
            scale,
        );

        // Energy Bar
        let energy_y = ui_y + bar_h + 40.0 * scale;
        UI::draw_bar(
            ui_x,
            energy_y,
            bar_w,
            bar_h,
            self.energy,
            "ENERGY",
            YELLOW,
            assets,
            scale,
        );

        // Score & Count
        UI::draw_score_and_popups(
            self.score,
            self.popups.len(),
            scale,
            offset_x,
            offset_y,
            assets,
        );
    }
}
