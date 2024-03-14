use macroquad::{
    audio::{load_sound, Sound},
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

    pub async fn load_fonts(fonts: &mut GameFonts) {
        let (body, body_italic, heading) = futures::join!(
            Self::load_font(BODY_FONT_PATH, "body font"),
            Self::load_font(BODY_ITALIC_FONT_PATH, "body italic font"),
            Self::load_font(HEADING_FONT_PATH, "heading font")
        );

        *fonts = GameFonts {
            body,
            body_italic,
            heading,
        };
    }

    async fn load_sound_asset(path: &str, description: &str) -> Option<Sound> {
        if let Ok(value) = load_sound(path).await {
            Some(value)
        } else {
            logging::error!("Failed to load {} from `{}`.", description, path);
            None
        }
    }

    pub async fn load_sounds(sounds: &mut GameSounds) {
        let (background, flap, game_over, obstacle_cleared, victory) = futures::join!(
            Self::load_sound_asset(BACKGROUND_SOUND_PATH, "background music sound"),
            Self::load_sound_asset(FLAP_SOUND_PATH, "flap sound"),
            Self::load_sound_asset(GAME_OVER_SOUND_PATH, "game over sound"),
            Self::load_sound_asset(OBSTACLE_CLEARED_SOUND_PATH, "obstacle cleared sound"),
            Self::load_sound_asset(VICTORY_SOUND_PATH, "victory sound")
        );
        *sounds = GameSounds {
            background,
            flap,
            game_over,
            obstacle_cleared,
            victory,
        };
    }
}
