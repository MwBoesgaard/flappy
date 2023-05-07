use macroquad::{prelude::*, window};

use crate::components::flappy::Flappy;
use crate::components::pillar::Pillar;
use crate::components::terrain::Terrain;
use crate::components::cloud::Cloud;
use crate::state::GameState;
use crate::state::WIDTH;
use crate::state::HEIGHT;

mod components;
mod state;

#[macroquad::main("Flappy")]
async fn main() {
    window::request_new_screen_size(WIDTH, HEIGHT);

    let logo_texture: Texture2D = load_texture("src/logo.png").await.unwrap();
    let flappy_sheet: Texture2D = load_texture("src/flappy-sheet.png").await.unwrap();
    let bg_texture: Texture2D = load_texture("src/bg.png").await.unwrap();
    let cloud_texture: Texture2D = load_texture("src/cloud-sheet.png").await.unwrap();

    let mut game_state: GameState = GameState::INTRO; 
    let mut flappy: Flappy = Flappy::new(150.0, 150.0, 50.0, 50.0, flappy_sheet);
    let mut pillars: Vec<Pillar> = vec![Pillar::new_random()];
    let mut terrain: Terrain = Terrain::new(bg_texture);
    let mut clouds: Vec<Cloud> = vec![Cloud::new(cloud_texture)];
    let mut score: u32 = 0;
    let mut high_score: u32 = 0;

    loop {
        clear_background(BLUE);

        match game_state
         {
            GameState::INTRO => {
                draw_texture_ex(logo_texture, WIDTH/2.0 - 250.0, HEIGHT * 0.1, WHITE, DrawTextureParams { dest_size: Some(vec2(500.0, 250.0)), source:None, rotation: (0.0), flip_x: (false), flip_y: (false), pivot: (None) });
                draw_texture_ex(flappy_sheet, WIDTH * 0.8, HEIGHT * 0.25, WHITE, DrawTextureParams { dest_size: Some(vec2(100.0, 100.0)), source: Some(Rect::new(50.0, 0.0, 50.0, 50.0)), rotation: (0.4), flip_x: (false), flip_y: (false), pivot: (None) });
                draw_texture_ex(flappy_sheet, WIDTH * 0.1, HEIGHT * 0.25, WHITE, DrawTextureParams { dest_size: Some(vec2(100.0, 100.0)), source: Some(Rect::new(0.0, 0.0, 50.0, 50.0)), rotation: (-0.4), flip_x: (false), flip_y: (false), pivot: (None) });
                draw_text("ENTER => Start", WIDTH/2.0 - 150.0, HEIGHT * 0.5 + 50.0, 26.0, DARKGRAY);
                draw_text("ESC => Exit", WIDTH/2.0 - 150.0, HEIGHT * 0.5 + 100.0, 26.0, DARKGRAY);
                draw_text("Pro-tip: Press arrow-up to not die", WIDTH/2.0 - 150.0, HEIGHT * 0.75, 26.0, ORANGE);
                draw_text(format!("High-Score: {high_score}").as_str(), WIDTH/2.0 - 150.0, HEIGHT * 0.82, 26.0, YELLOW);

                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::PLAYING;
                }

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::STOPPED;
                }
            },
            GameState::PLAYING => {
                // Draw elements
                terrain.draw();
              
                for cloud in &clouds {
                    cloud.draw();
                }

                flappy.draw();

                for pillar in &pillars {
                    pillar.draw();
                }

                draw_text(format!("Score: {score}").as_str(), WIDTH * 0.85, HEIGHT * 0.1, 24.0, YELLOW);

                // Check for key commands
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::STOPPED;
                }

                if is_key_down(KeyCode::Up) {
                    flappy.jump();
                }

                // Move game objects according to their acceleration
                flappy.acc_based_movement();

                for pillar in &mut pillars {
                    pillar.acc_based_movement();
                }

                for cloud in &mut clouds {
                    cloud.acc_based_movement();
                }

                terrain.acc_based_movement();

                // Apply gravity
                flappy.fall();

                // Remove and add pillars as needed
                pillars.retain(|p| p.x > -50.0);

                if pillars.len() < 4 && pillars.iter().any(|p| p.x == WIDTH/2.0) {
                    pillars.push(Pillar::new_random());
                }

                // Update score and high-score
                if pillars.iter().any(|p| p.x + p.width == flappy.x) {
                    score += 1;

                    if score != 0 && score % 2 == 0 {
                        clouds.push(Cloud::new(cloud_texture)); // Generate new clouds based on score.
                    }
                }

                if score > high_score {
                    high_score = score;
                }

                // Check for collisions and out of bounds
                if flappy.is_out_of_bounds(HEIGHT) {
                    reset_game(&mut game_state, &mut flappy, &mut pillars, &mut score, &mut terrain, &mut clouds);
                }

                if flappy.collision(&pillars) {
                    reset_game(&mut game_state, &mut flappy, &mut pillars, &mut score, &mut terrain, &mut clouds);
                }
            },
            GameState::STOPPED => {
                break;
            }
        }
        
        next_frame().await
    }
}

pub fn reset_game(game_state: &mut GameState, flappy: &mut Flappy, pillars: &mut Vec<Pillar>, score: &mut u32, terrain: &mut Terrain, clouds: &mut Vec<Cloud>) {
    *game_state = GameState::INTRO;
    *flappy = Flappy::new(150.0, 150.0, 50.0, 50.0, flappy.texture);
    *pillars = vec![Pillar::new_random()];
    *score = 0;
    *terrain = Terrain::new(terrain.texture);
    *clouds = vec![Cloud::new(clouds.first().unwrap().texture)];
}
