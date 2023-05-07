use macroquad::{texture::{Texture2D, draw_texture_ex, DrawTextureParams}, prelude::{WHITE, Rect}};

use super::super::state::WIDTH;

pub struct Terrain {
    pub x: f32,
    pub ax: f32,
    pub texture: Texture2D
}

impl Terrain {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            x: 0.0,
            ax: 0.5,
            texture
        }
    }

    pub fn acc_based_movement(&mut self) {
        self.x += self.ax;
    }

    pub fn draw(&self) {
        draw_texture_ex(self.texture, 0.0, 500.0, WHITE, DrawTextureParams { dest_size: None, source: Some(Rect::new(self.x, 0.0, WIDTH, 100.0)), rotation: (0.0), flip_x: (false), flip_y: (false), pivot: (None) });
    }
}