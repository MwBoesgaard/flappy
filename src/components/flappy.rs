use macroquad::{prelude::{WHITE, vec2, Rect}, texture::{DrawTextureParams, draw_texture_ex, Texture2D}};

use super::pillar::Pillar;

pub struct Flappy {
    pub x: f32,
    pub y: f32,
    pub ay: f32,
    pub width: f32,
    pub height: f32,
    pub texture: Texture2D
}

impl Flappy {
    pub fn new(x: f32, y: f32, width: f32, height: f32, texture: Texture2D) -> Self {
        Self {
            x,
            y,
            ay: 0.0,
            width,
            height,
            texture,
        }
    }

    pub fn jump(&mut self) {
        if self.ay > -5.0 {
            self.ay -= 20.0;
        }
    }

    pub fn fall(&mut self) {
        if self.ay < 15.0 {
            self.ay += 3.0;
        }
    }

    pub fn acc_based_movement(&mut self) {
        self.y += self.ay;
    }

    pub fn is_out_of_bounds(&mut self, height: f32) -> bool {
        return self.y >= height || self.y <= 0.0 - self.height;
    }

    pub fn collision(&self, pillars: &Vec<Pillar>) -> bool {
        for pillar in pillars {
            if self.x + self.width >= pillar.x && self.x <= pillar.x + pillar.width {
                if self.y < pillar.gap_start || self.y + self.height > pillar.gap_start + pillar.gap_height {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn draw(&self) {
        if self.ay < 0.0 {
            draw_texture_ex(self.texture, self.x, self.y, WHITE, DrawTextureParams { dest_size: Some(vec2(50.0, 50.0)), source: Some(Rect::new(0.0, 0.0, 50.0, 50.0)), rotation: (-0.2), flip_x: (false), flip_y: (false), pivot: (None) });
        } else {
            draw_texture_ex(self.texture, self.x, self.y, WHITE, DrawTextureParams { dest_size: Some(vec2(50.0, 50.0)), source: Some(Rect::new(50.0, 0.0, 50.0, 50.0)), rotation: (0.2), flip_x: (false), flip_y: (false), pivot: (None) });
        }    
    }

 
}