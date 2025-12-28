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

// Draw a teardrop shape pointing in a direction
fn draw_teardrop(x: f32, y: f32, size: f32, direction: f32, color: Color) {
    // Create teardrop pointing in the given direction (in radians)
    let cos_d = direction.cos();
    let sin_d = direction.sin();
    
    // Teardrop shape: pointed tip and rounded back
    let tip = vec2(x + cos_d * size, y + sin_d * size);
    
    // Create rounded back using two points perpendicular to direction
    let perp_cos = -sin_d;
    let perp_sin = cos_d;
    let back_left = vec2(x - perp_cos * size * 0.6, y - perp_sin * size * 0.6);
    let back_right = vec2(x + perp_cos * size * 0.6, y + perp_sin * size * 0.6);
    
    // Draw teardrop as triangle
    draw_triangle(tip, back_left, back_right, color);
    draw_triangle_lines(tip, back_left, back_right, 1.5, color);
}

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
    pub cursor_trail: Vec<(f32, f32, f32)>, // (x, y, age)
    pub pulse_timer: f32,
    pub last_mouse_x: f32,
    pub last_mouse_y: f32,
    pub arrow_alpha: f32,
    pub movement_direction: f32, // Direction in radians
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
            cursor_trail: Vec::new(),
            pulse_timer: 0.0,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
            arrow_alpha: 1.0,
            movement_direction: 0.0,
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
        self.cursor_trail.clear();
        self.arrow_alpha = 1.0;
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

        // Update pulse timer for cursor glow
        self.pulse_timer += dt;

        // Update cursor trail
        let (raw_mx, raw_my) = mouse_position();
        self.cursor_trail.push((raw_mx, raw_my, 0.0));
        for trail_point in self.cursor_trail.iter_mut() {
            trail_point.2 += dt;
        }
        self.cursor_trail.retain(|p| p.2 < 0.3); // Keep trail for 0.3 seconds

        // Track mouse movement for arrow visibility and direction
        let dx = raw_mx - self.last_mouse_x;
        let dy = raw_my - self.last_mouse_y;
        let movement_speed = (dx * dx + dy * dy).sqrt();
        
        // Calculate movement direction (angle in radians)
        if movement_speed > 0.1 {
            self.movement_direction = dy.atan2(dx); // atan2(y, x) gives the angle
        }
        
        // Convert movement speed to arrow alpha (fast = visible, slow = fade)
        let target_alpha = if movement_speed > 1.0 {
            1.0 // Fast movement = arrow visible
        } else {
            0.0 // No movement = arrow fade
        };
        
        // Smooth interpolation for alpha (fade in/out smoothly)
        self.arrow_alpha = self.arrow_alpha + (target_alpha - self.arrow_alpha) * (dt * 3.0).min(1.0);
        
        // Update last mouse position
        self.last_mouse_x = raw_mx;
        self.last_mouse_y = raw_my;

        // Get scaling
        let (scale, offset_x, offset_y) = self.get_scaling();
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

        // Custom cursor - Light from Teardrop
        let cursor_center_x = raw_mx;
        let cursor_center_y = raw_my;

        // Draw cursor trail from center of light
        for (x, y, age) in self.cursor_trail.iter() {
            let alpha = 1.0 - (age / 0.3); // Fade out over time
            let trail_radius = 4.0 * scale * alpha;
            // Trail comes from center of screen, not cursor position
            let dx = *x - cursor_center_x;
            let dy = *y - cursor_center_y;
            let trail_x = cursor_center_x + dx * 0.3; // Reduced distance for center origin
            let trail_y = cursor_center_y + dy * 0.3;
            draw_circle(trail_x, trail_y, trail_radius, Color::new(1.0, 0.8, 0.0, alpha * 0.5));
        }

        // Draw light effect as a simple glow (no circles)
        let light_radius = 60.0 * scale;
        for i in 1..=5 {
            let radius = (light_radius * i as f32 / 5.0) as f32;
            let alpha = 0.06 * (1.0 - (i as f32 / 5.0).powi(2));
            draw_circle(cursor_center_x, cursor_center_y, radius, Color::new(1.0, 0.9, 0.4, alpha));
        }

        // Bright core light
        draw_circle(cursor_center_x, cursor_center_y, 10.0 * scale, Color::new(1.0, 0.95, 0.6, 0.6));
        draw_circle(cursor_center_x, cursor_center_y, 6.0 * scale, Color::new(1.0, 1.0, 0.8, 0.8));

        // Draw the teardrop cursor with dynamic alpha based on movement
        let teardrop_color = Color::new(1.0, 0.9, 0.4, self.arrow_alpha); // Warm golden yellow
        draw_teardrop(cursor_center_x, cursor_center_y, 12.0 * scale, self.movement_direction, teardrop_color);
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
