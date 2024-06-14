use bevy_ecs::{entity::Entity, system::Resource};
use macroquad::{audio::Sound, text::Font};

use crate::WINDOW_WIDTH;

#[derive(Debug, PartialEq, Resource)]
pub enum ResumeGameMode {
    Menu,
    Playing,
}

#[derive(Debug, Default, PartialEq, Resource)]
pub enum GameMode {
    Exiting(ResumeGameMode),
    Menu,
    Playing,
    GameOver,

    #[default]
    Title,

    Won,
}

#[derive(Debug, Default, Resource)]
pub struct GameState {
    pub mode: GameMode,
}

#[derive(Default, Resource)]
pub struct GameFonts {
    pub body: Option<Font>,
    pub body_italic: Option<Font>,
    pub heading: Option<Font>,
}

#[derive(Default, Resource)]
pub struct GameSounds {
    pub background: Option<Sound>,
    pub flap: Option<Sound>,
    pub game_over: Option<Sound>,
    pub obstacle_cleared: Option<Sound>,
    pub victory: Option<Sound>,
}

#[derive(Default, Resource)]
pub struct GameAssets {
    pub fonts: GameFonts,
    pub sounds: GameSounds,
}

#[derive(Resource)]
pub struct Camera {
    pub pan_speed: f32,
    pub left_displacement: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pan_speed: 240.0,
            left_displacement: 0.0,
        }
    }
}

impl Camera {
    pub fn in_view(&self, x_displacement: f32) -> bool {
        self.left_displacement < x_displacement
            && x_displacement < self.left_displacement + WINDOW_WIDTH
    }

    pub fn update(&mut self, delta: f32) {
        self.left_displacement += delta * self.pan_speed;
    }
}

#[derive(Resource, Default)]
pub struct DeltaTime {
    pub seconds: f32,
}

#[derive(Resource, Default)]
pub struct ClearedObstacles {
    pub obstacles: Vec<Entity>,
}
