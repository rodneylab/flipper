use macroquad::{
    audio::load_sound,
    logging,
    text::{load_ttf_font, Font},
};

use crate::resources::{GameFonts, GameSounds};

#[derive(Default)]
pub struct AssetManager;

const BODY_FONT_PATH: &str = "./assets/overpass-v13-latin-regular.ttf";
const HEADING_FONT_PATH: &str = "./assets/pacifico-v22-latin-regular.ttf";
const BODY_ITALIC_FONT_PATH: &str = "./assets/overpass-v13-latin-italic.ttf";

const BACKGROUND_SOUND_PATH: &str = "./assets/background.wav";
const FLAP_SOUND_PATH: &str = "./assets/flap.wav";
const GAME_OVER_SOUND_PATH: &str = "./assets/game_over.wav";
const OBSTACLE_CLEARED_SOUND_PATH: &str = "./assets/obstacle_cleared.wav";
const VICTORY_SOUND_PATH: &str = "./assets/victory.wav";

impl AssetManager {
    async fn load_font(path: &str, description: &str) -> Option<Font> {
        if let Ok(value) = load_ttf_font(path).await {
            Some(value)
        } else {
            logging::error!("Failed to load {} from `{}`.", description, path);
            None
        }
    }

    pub async fn load_fonts(&mut self, fonts: &mut GameFonts) {
        let body = Self::load_font(BODY_FONT_PATH, "body font").await;
        let body_italic = Self::load_font(BODY_ITALIC_FONT_PATH, "body italic font").await;
        let heading = Self::load_font(HEADING_FONT_PATH, "heading font").await;

        *fonts = GameFonts {
            body,
            body_italic,
            heading,
        };
    }

    pub async fn load_sounds(sounds: &mut GameSounds) {
        let background = if let Ok(value) = load_sound(BACKGROUND_SOUND_PATH).await {
            Some(value)
        } else {
            None
        };
        let flap = if let Ok(value) = load_sound(FLAP_SOUND_PATH).await {
            Some(value)
        } else {
            None
        };
        let game_over = if let Ok(value) = load_sound(GAME_OVER_SOUND_PATH).await {
            Some(value)
        } else {
            None
        };
        let obstacle_cleared = if let Ok(value) = load_sound(OBSTACLE_CLEARED_SOUND_PATH).await {
            Some(value)
        } else {
            None
        };
        let victory = if let Ok(value) = load_sound(VICTORY_SOUND_PATH).await {
            Some(value)
        } else {
            None
        };

        *sounds = GameSounds {
            background,
            flap,
            game_over,
            obstacle_cleared,
            victory,
        };
    }
}
