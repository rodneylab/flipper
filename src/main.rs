#![warn(clippy::all, clippy::pedantic)]
use macroquad::{
    color::colors::{DARKBLUE, SKYBLUE, YELLOW},
    input::{is_key_down, is_key_released, KeyCode},
    shapes::draw_rectangle,
    text::draw_text,
    window::{clear_background, next_frame, screen_height},
};

struct Flipper {
    x_displacement: f32,
    y_displacement: f32,
    acceleration: f32,
    velocity: f32,
}

impl Default for Flipper {
    fn default() -> Self {
        Flipper {
            x_displacement: 20.0,
            y_displacement: 0.5 * screen_height() - 10.0,
            acceleration: -0.5,
            velocity: 0.0,
        }
    }
}

impl Flipper {
    fn draw(&self) {
        draw_rectangle(
            self.x_displacement,
            self.y_displacement,
            20.0,
            20.0,
            SKYBLUE,
        );
    }

    fn flap(&mut self) {
        if self.velocity > -3.0 {
            self.velocity += self.acceleration;
        }
    }

    fn update(&mut self) -> GameMode {
        // gravity
        if self.velocity < 0.5 {
            self.velocity += 0.1;
        }

        if self.y_displacement <= 0.0 {
            self.y_displacement = 0.0;
        }
        if self.y_displacement > screen_height() {
            //self.y_displacement = screen_height() - 20.0;
            return GameMode::GameOver;
        }
        self.y_displacement += self.velocity;

        GameMode::Playing
    }
}

#[derive(Default)]
enum GameMode {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Default)]
struct GameState {
    flipper: Flipper,
    mode: GameMode,
}

#[macroquad::main("Flipper")]
async fn main() {
    let mut game_state = GameState::default();

    loop {
        //        let GameState {
        //            ref mut flipper, ..
        //        } = game_state;

        match game_state.mode {
            GameMode::Menu => {
                clear_background(YELLOW);
                draw_text("Press SPACE to play", 20.0, 20.0, 30.0, DARKBLUE);
                if is_key_down(KeyCode::Space) {
                    game_state.mode = GameMode::Playing;
                }
                if is_key_down(KeyCode::Escape) {
                    break;
                }
            }
            GameMode::Playing => {
                let GameState {
                    ref mut flipper, ..
                } = game_state;
                clear_background(DARKBLUE);
                draw_text("Press SPACE to soar", 20.0, 20.0, 30.0, YELLOW);

                if is_key_down(KeyCode::Space) {
                    flipper.flap();
                }
                if is_key_down(KeyCode::Escape) {
                    break;
                }

                game_state.mode = flipper.update();

                flipper.draw();
            }
            GameMode::GameOver => {
                clear_background(YELLOW);
                draw_text(
                    "Game Over! Press SPACE to play again",
                    20.0,
                    20.0,
                    30.0,
                    DARKBLUE,
                );

                if is_key_released(KeyCode::Space) {
                    game_state.flipper = Flipper::default();
                    game_state.mode = GameMode::Menu;
                }
            }
        }

        next_frame().await;
    }
}
