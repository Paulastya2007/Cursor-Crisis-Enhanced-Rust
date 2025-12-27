use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub struct Popup {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub speed: f32,
    pub char_index: usize,
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
        }
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
            draw_texture_ex(
                tex,
                ox + self.x * scale,
                oy + self.y * scale,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.w * scale, self.h * scale)),
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
