use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub struct Popup {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub speed: f32,
    pub char_index: usize,
    pub scale_timer: f32,
}

impl Popup {
    pub fn new(max_w: f32, max_h: f32, num_chars: usize) -> Self {
        let size = gen_range(40.0, 60.0);
        Self {
            x: gen_range(0.0, max_w - size),
            y: gen_range(0.0, max_h - size),
            w: size,
            h: size,
            speed: gen_range(40.0, 110.0),
            char_index: gen_range(0, num_chars),
            scale_timer: gen_range(0.0, 2.0 * std::f32::consts::PI), // Random start phase
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.scale_timer += dt;
    }

    pub fn get_scale_multiplier(&self) -> f32 {
        // Pulsing effect: scales between 0.85 and 1.15
        let pulse = (self.scale_timer * 5.0).sin();
        0.85 + (pulse + 1.0) * 0.15
    }
    pub fn follow(&mut self, target_x: f32, target_y: f32, dt: f32) {
        let dx = target_x - (self.x + self.w / 2.0);
        let dy = target_y - (self.y + self.h / 2.0);

        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 1.0 {
            self.x += (dx / dist) * self.speed * dt;
            self.y += (dy / dist) * self.speed * dt;
        }
    }

    pub fn hit(&self, mx: f32, my: f32) -> bool {
        mx > self.x && mx < self.x + self.w && my > self.y && my < self.y + self.h
    }

    pub fn draw_scaled(&self, scale: f32, ox: f32, oy: f32, char_textures: &[Texture2D]) {
        if let Some(tex) = char_textures.get(self.char_index) {
            let scale_mult = self.get_scale_multiplier();
            let scaled_w = self.w * scale_mult;
            let scaled_h = self.h * scale_mult;
            let offset_x = (self.w - scaled_w) / 2.0; // Center the scaling
            let offset_y = (self.h - scaled_h) / 2.0;

            draw_texture_ex(
                tex,
                ox + (self.x + offset_x) * scale,
                oy + (self.y + offset_y) * scale,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(scaled_w * scale, scaled_h * scale)),
                    ..Default::default()
                },
            );
        } else {
            // Fallback to rectangle if texture index is invalid
            draw_rectangle(
                ox + self.x * scale,
                oy + self.y * scale,
                self.w * scale,
                self.h * scale,
                GRAY,
            );
        }
    }
}
