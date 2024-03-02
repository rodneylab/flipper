#![warn(clippy::all, clippy::pedantic)]

mod entities;
mod fonts;
mod sound;
mod ui;

use entities::{flipper_finish_line_collision, Camera, FinishLine, Flipper, Obstacle};
use fonts::{load_body_font, load_body_italic_font, load_heading_font};
use macroquad::{
    audio::Sound,
    input::{
        is_key_down, is_key_released, is_mouse_button_pressed, is_quit_requested, prevent_quit,
        KeyCode, MouseButton,
    },
    logging,
    time::{get_frame_time, get_time},
    window::{clear_background, next_frame, Conf},
};
use sound::{
    load_background as load_background_sound, load_flap as load_flap_sound,
    load_game_over as load_game_over_sound, load_victory as load_victory_sound, play_sound_once,
    start_playing_looped as start_playing_looped_sound,
    stop_playing_looped as stop_playing_looped_sound,
};
use ui::{
    draw_exit_screen_text, draw_game_over_screen_text, draw_info_text, draw_menu_screen_text,
    draw_title_screen_text, draw_win_screen_text, COLUMBIABLUE, DARKPASTELGREEN, DEEPSKYBLUE,
    MAIZE, YINMNBLUE,
};

use crate::entities::flipper_obstacle_collision;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

#[derive(Debug, PartialEq)]
enum ResumeGameMode {
    Menu,
    Playing,
}

#[derive(Debug, Default, PartialEq)]
enum GameMode {
    Exiting(ResumeGameMode),
    Menu,
    Playing,
    GameOver,

    #[default]
    Title,

    Won,
}

#[derive(Default)]
struct GameState {
    camera: Camera,
    finish_line: FinishLine,
    flipper: Flipper,
    mode: GameMode,
    obstacles: Vec<Obstacle>,
}

impl GameState {
    fn reset(&mut self) {
        self.camera = Camera::default();
        self.flipper = Flipper::default();
    }
}

fn conf() -> Conf {
    #[allow(clippy::cast_possible_truncation)]
    Conf {
        window_title: String::from("Flipper"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

fn handle_start_game(background_sound: &Sound) -> Option<GameMode> {
    if is_key_down(KeyCode::Space) {
        start_playing_looped_sound(background_sound);
        return Some(GameMode::Playing);
    }
    None
}

fn handle_request_quit(resume_mode: ResumeGameMode) -> Option<GameMode> {
    if is_key_released(KeyCode::Escape) || is_quit_requested() {
        return Some(GameMode::Exiting(resume_mode));
    }
    None
}

fn handle_replay(game_state: &mut GameState) {
    if is_key_released(KeyCode::Space) {
        game_state.reset();
        game_state.mode = GameMode::Menu;
    }
}

fn handle_skip_title() -> Option<GameMode> {
    if is_key_released(KeyCode::Space)
        || is_mouse_button_pressed(MouseButton::Left)
        || get_time() > 5.0
    {
        return Some(GameMode::Menu);
    }
    None
}

fn generate_obscructions() -> Vec<Obstacle> {
    let result: Vec<Obstacle> = vec![
        Obstacle::new(600.0, 300.0, 100.0),
        Obstacle::new(900.0, 350.0, 75.0),
        Obstacle::new(1200.0, 300.0, 75.0),
    ];

    result
}

async fn handle_before_transisition_to_won(background_sound: &Sound) {
    stop_playing_looped_sound(background_sound);
    let victory_sound = load_victory_sound().await;
    play_sound_once(&victory_sound);
}

async fn handle_before_transisition_to_game_over(background_sound: &Sound) {
    stop_playing_looped_sound(background_sound);
    let game_over_sound = load_game_over_sound().await;
    play_sound_once(&game_over_sound);
}

async fn handle_collisions(
    flipper: &Flipper,
    obstacles: &[Obstacle],
    finish_line: &FinishLine,
    background_sound: &Sound,
) -> Option<GameMode> {
    if obstacles
        .iter()
        .any(|val| flipper_obstacle_collision(flipper, val))
    {
        handle_before_transisition_to_game_over(background_sound).await;
        return Some(GameMode::GameOver);
    }
    if flipper_finish_line_collision(flipper, finish_line) {
        handle_before_transisition_to_won(background_sound).await;
        return Some(GameMode::Won);
    }
    None
}

#[macroquad::main(conf)]
async fn main<'a>() {
    prevent_quit();

    let heading_font = load_heading_font().await;
    let body_italic_font = load_body_italic_font().await;
    let body_font = load_body_font().await;

    let mut game_state = GameState::default();
    let background_sound = load_background_sound().await;
    game_state.flipper.with_flap_sound(load_flap_sound().await);
    game_state.obstacles = generate_obscructions();

    loop {
        let delta = get_frame_time();

        logging::trace!("Game mode is {:?}", game_state.mode);

        match game_state.mode {
            GameMode::Exiting(ref resume_mode) => {
                clear_background(MAIZE);
                draw_exit_screen_text(&body_font);
                if is_key_down(KeyCode::Enter) {
                    break;
                }
                if is_key_released(KeyCode::Escape) {
                    match resume_mode {
                        ResumeGameMode::Playing => game_state.mode = GameMode::Playing,
                        ResumeGameMode::Menu => game_state.mode = GameMode::Menu,
                    }
                }
            }
            GameMode::Title => {
                clear_background(MAIZE);
                draw_title_screen_text(&heading_font, &body_font, &body_italic_font);
                if let Some(value) = handle_skip_title() {
                    game_state.mode = value;
                }
                if let Some(value) = handle_request_quit(ResumeGameMode::Menu) {
                    game_state.mode = value;
                };
            }
            GameMode::Menu => {
                clear_background(DARKPASTELGREEN);
                draw_menu_screen_text(&body_font);
                if let Some(value) = handle_start_game(&background_sound) {
                    game_state.mode = value;
                }
                if let Some(value) = handle_request_quit(ResumeGameMode::Menu) {
                    game_state.mode = value;
                };
            }
            GameMode::Playing => {
                let GameState {
                    ref mut camera,
                    ref finish_line,
                    ref mut flipper,
                    ref obstacles,
                    ..
                } = game_state;
                clear_background(DEEPSKYBLUE);
                draw_info_text(&body_font);

                if let Some(value) = handle_request_quit(ResumeGameMode::Playing) {
                    game_state.mode = value;
                } else {
                    if is_key_down(KeyCode::Space) {
                        flipper.flap();
                    }

                    camera.update(delta);
                    if let Some(value) =
                        handle_collisions(flipper, obstacles, finish_line, &background_sound).await
                    {
                        game_state.mode = value;
                    } else if let Some(value) = flipper.update(delta) {
                        if value == GameMode::GameOver {
                            handle_before_transisition_to_game_over(&background_sound).await;
                        }
                        game_state.mode = value;
                    }
                    flipper.draw(camera);
                    obstacles.iter().for_each(|val| val.draw(camera));
                    finish_line.draw(camera);
                }
            }
            GameMode::GameOver => {
                clear_background(COLUMBIABLUE);
                draw_game_over_screen_text(&body_font);

                handle_replay(&mut game_state);
                if let Some(value) = handle_request_quit(ResumeGameMode::Menu) {
                    game_state.mode = value;
                };
            }
            GameMode::Won => {
                clear_background(YINMNBLUE);
                draw_win_screen_text(&body_font);

                handle_replay(&mut game_state);
                if let Some(value) = handle_request_quit(ResumeGameMode::Menu) {
                    game_state.mode = value;
                };
            }
        }

        next_frame().await;
    }
}
