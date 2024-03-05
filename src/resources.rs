use bevy_ecs::system::Resource;
use macroquad::audio::Sound;

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
    pub background_sound: Option<Sound>,
    pub game_over_sound: Option<Sound>,
    pub victory_sound: Option<Sound>,
    pub is_quit_requested: bool,
    pub mode: GameMode,
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
