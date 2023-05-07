use macroquad::{prelude::*, color};

use super::super::state::{WIDTH, HEIGHT};

pub struct Pillar {
    pub x: f32,
    ax: f32,
    pub width: f32,
    pub gap_start: f32,
    pub gap_height: f32,
    pub color: Color
}

impl Pillar {
    pub fn new(gap_start: f32, gap_height: f32) -> Self {
        return Self {
            x: WIDTH + 50.0,
            ax: -5.0,
            width: 25.0,
            gap_start: gap_start,
            gap_height: gap_height,
            color: color::BROWN
        }
    }

    pub fn new_random() -> Self {
        let gap_start = rand::gen_range(50, 350);
        let gap_height = rand::gen_range(150, 250);

        return Self::new(gap_start as f32, gap_height as f32);
    }

    pub fn acc_based_movement(&mut self) {
        self.x += self.ax;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, 0.0, self.width, self.gap_start, self.color); //Top
        draw_rectangle(self.x, self.gap_start + self.gap_height, self.width, HEIGHT - self.gap_start + self.gap_height, self.color); // Bottom
    }
}