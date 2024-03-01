#![warn(clippy::all, clippy::pedantic)]

mod fonts;
mod sound;
mod ui;

use fonts::{load_body_font, load_body_italic_font, load_heading_font};
use macroquad::{
    audio::{play_sound, stop_sound, PlaySoundParams, Sound},
    input::{
        is_key_down, is_key_released, is_mouse_button_pressed, is_quit_requested, prevent_quit,
        KeyCode, MouseButton,
    },
    logging,
    shapes::draw_rectangle,
    time::{get_frame_time, get_time},
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};
use sound::{
    load_background as load_background_sound, load_flap as load_flap_sound,
    load_game_over as load_game_over_sound, load_victory as load_victory_sound,
};

use ui::{
    draw_exit_screen_text, draw_game_over_screen_text, draw_info_text, draw_menu_screen_text,
    draw_title_screen_text, draw_win_screen_text, COLUMBIABLUE, DARKPASTELGREEN, DEEPSKYBLUE,
    MAIZE, YINMNBLUE,
};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

struct Camera {
    pan_speed: f32,
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
    fn in_view(&self, x_displacement: f32) -> bool {
        self.left_displacement < x_displacement && x_displacement < x_displacement + screen_width()
    }

    fn update(&mut self, delta: f32) {
        self.left_displacement += delta * self.pan_speed;
    }
}

struct FinishLine {
    x_displacement: f32,
    width: f32,
}

impl Default for FinishLine {
    fn default() -> Self {
        FinishLine {
            width: 5.0,
            x_displacement: 4.0 * WINDOW_WIDTH,
        }
    }
}

impl FinishLine {
    fn draw(&self, camera: &Camera) {
        if camera.in_view(self.x_displacement) {
            draw_rectangle(
                self.x_displacement - camera.left_displacement,
                0.0,
                5.0,
                WINDOW_HEIGHT,
                COLUMBIABLUE,
            );
        }
    }

    fn left(&self) -> f32 {
        self.x_displacement
    }

    fn right(&self) -> f32 {
        self.x_displacement + self.width
    }

    #[allow(dead_code)]
    fn with_x_displacement(&mut self, new_x_displacement: f32) -> &mut Self {
        self.x_displacement = new_x_displacement;
        self
    }

    #[allow(dead_code)]
    fn with_width(&mut self, new_width: f32) -> &mut Self {
        self.width = new_width;
        self
    }
}

struct Flipper {
    flap_sound: Option<Sound>,
    x_displacement: f32,
    y_displacement: f32,
    acceleration: f32,
    x_velocity: f32,
    y_velocity: f32,
    width: f32,
}

impl Default for Flipper {
    fn default() -> Self {
        Flipper {
            flap_sound: None,
            x_displacement: 20.0,
            y_displacement: 0.5 * WINDOW_HEIGHT - 10.0,
            acceleration: -30.0,
            x_velocity: 240.0,
            y_velocity: 0.0,
            width: 20.0,
        }
    }
}

impl Flipper {
    fn draw(&self, camera: &Camera) {
        draw_rectangle(
            self.x_displacement - camera.left_displacement,
            self.y_displacement,
            self.width,
            20.0,
            YINMNBLUE,
        );
    }

    fn flap(&mut self) {
        if let Some(value) = &self.flap_sound {
            play_sound(
                value,
                PlaySoundParams {
                    looped: false,
                    volume: 0.05,
                },
            );
        };
        if self.y_velocity > -180.0 {
            self.y_velocity += self.acceleration;
        }
    }

    fn left(&self) -> f32 {
        self.x_displacement
    }

    fn right(&self) -> f32 {
        self.x_displacement + self.width
    }

    fn update(&mut self, delta: f32) -> Option<GameMode> {
        self.x_displacement += delta * self.x_velocity;

        // gravity
        if self.y_velocity < 30.0 {
            self.y_velocity += 6.0;
        }

        if self.y_displacement <= 0.0 {
            self.y_displacement = 0.0;
        } else if self.y_displacement > screen_height() {
            return Some(GameMode::GameOver);
        }
        self.y_displacement += delta * self.y_velocity;

        None
    }

    fn with_flap_sound(&mut self, sound: Sound) -> &mut Self {
        self.flap_sound = Some(sound);
        self
    }

    #[allow(dead_code)]
    fn with_x_displacement(&mut self, new_x_displacement: f32) -> &mut Self {
        self.x_displacement = new_x_displacement;
        self
    }

    #[allow(dead_code)]
    fn with_width(&mut self, new_width: f32) -> &mut Self {
        self.width = new_width;
        self
    }
}

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
}

impl GameState {
    fn reset(&mut self) {
        self.camera = Camera::default();
        self.flipper = Flipper::default();
    }
}

fn flipper_finish_line_collision(flipper: &Flipper, finish_line: &FinishLine) -> bool {
    flipper.right() > finish_line.left() && flipper.left() < finish_line.right()
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

fn play_sound_once(sound: &Sound) {
    play_sound(
        sound,
        PlaySoundParams {
            looped: false,
            volume: 0.05,
        },
    );
}

fn start_playing_looped_sound(sound: &Sound) {
    play_sound(
        sound,
        PlaySoundParams {
            looped: true,
            volume: 0.05,
        },
    );
}

fn stop_playing_looped_sound(sound: &Sound) {
    stop_sound(sound);
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

#[macroquad::main(conf)]
async fn main<'a>() {
    prevent_quit();

    let heading_font = load_heading_font().await;
    let body_italic_font = load_body_italic_font().await;
    let body_font = load_body_font().await;

    let mut game_state = GameState::default();
    let background_sound = load_background_sound().await;
    game_state.flipper.with_flap_sound(load_flap_sound().await);

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
                    if flipper_finish_line_collision(flipper, finish_line) {
                        game_state.mode = GameMode::Won;
                        stop_playing_looped_sound(&background_sound);
                        let victory_sound = load_victory_sound().await;
                        play_sound_once(&victory_sound);
                    } else if let Some(value) = flipper.update(delta) {
                        if value == GameMode::GameOver {
                            stop_playing_looped_sound(&background_sound);
                            let game_over_sound = load_game_over_sound().await;
                            play_sound_once(&game_over_sound);
                        }
                        game_state.mode = value;
                    }
                    flipper.draw(camera);
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

#[cfg(test)]
mod tests {
    use super::{flipper_finish_line_collision, FinishLine, Flipper};
    use float_cmp::approx_eq;

    #[test]
    fn finish_line_left_returns_expected_value() {
        // arrange
        let mut finish_line = FinishLine::default();
        finish_line.with_x_displacement(50.0).with_width(10.0);

        // act
        let result = finish_line.left();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            50.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn finish_line_right_returns_expected_value() {
        // arrange
        let mut finish_line = FinishLine::default();
        finish_line.with_x_displacement(50.0).with_width(10.0);

        // act
        let result = finish_line.right();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            60.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn flipper_left_returns_expected_value() {
        // arrange
        let mut flipper = Flipper::default();
        flipper.with_x_displacement(100.0).with_width(30.0);

        // act
        let result = flipper.left();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            100.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn flipper_right_returns_expected_value() {
        // arrange
        let mut flipper = Flipper::default();
        flipper.with_x_displacement(100.0).with_width(30.0);

        // act
        let result = flipper.right();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            130.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn flipper_finish_line_collision_successfully_reports_miss() {
        // arrange
        let mut flipper = Flipper::default();
        flipper.with_x_displacement(0.0);

        let mut finish_line = FinishLine::default();
        finish_line.with_x_displacement(100.0);

        // act
        let result = flipper_finish_line_collision(&flipper, &finish_line);

        // assert
        assert!(!result);
    }

    #[test]
    fn flipper_finish_line_collision_successfully_reports_collision() {
        // arrange
        let mut flipper = Flipper::default();
        flipper.with_x_displacement(100.0);

        let mut finish_line = FinishLine::default();
        finish_line.with_x_displacement(100.0);

        // act
        let result = flipper_finish_line_collision(&flipper, &finish_line);

        // assert
        assert!(result);
    }
}
