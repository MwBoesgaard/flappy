use macroquad::{texture::{Texture2D, draw_texture_ex, DrawTextureParams}, prelude::{WHITE, Rect, rand}};

use super::super::state::WIDTH;

pub struct Cloud {
    pub x: f32,
    pub y: f32,
    pub ax: f32,
    pub ver: i32,
    pub texture: Texture2D
}

impl Cloud {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            x: WIDTH,
            y: rand::gen_range(50, 500) as f32,
            ax: -1.0,
            ver: rand::gen_range(0, 4),
            texture
        }
    }

    pub fn acc_based_movement(&mut self) {
        self.x += self.ax;
    }

    pub fn draw(&self) {
        draw_texture_ex(self.texture, self.x, self.y, WHITE, DrawTextureParams { dest_size: None, source: Some(Rect::new(self.ver as f32 * 100.0, 0.0, 100.0, 50.0)), rotation: (0.0), flip_x: (false), flip_y: (false), pivot: (None) });
    }
}