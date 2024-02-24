#![warn(clippy::all, clippy::pedantic)]
use macroquad::{
    color::colors::{DARKBLUE, SKYBLUE, WHITE, YELLOW},
    input::{is_key_down, is_key_released, KeyCode},
    shapes::draw_rectangle,
    text::draw_text,
    window::{clear_background, next_frame, screen_height, screen_width},
};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct Camera {
    pan_speed: f32,
    pub left_displacement: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pan_speed: 4.0,
            left_displacement: 0.0,
        }
    }
}

impl Camera {
    fn in_view(&self, x_displacement: f32) -> bool {
        self.left_displacement < x_displacement && x_displacement < x_displacement + screen_width()
    }

    fn update(&mut self) {
        self.left_displacement += self.pan_speed;
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
            x_displacement: 4.0 * SCREEN_WIDTH,
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
                screen_height(),
                WHITE,
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
            x_displacement: 20.0,
            y_displacement: 0.5 * SCREEN_HEIGHT - 10.0,
            acceleration: -0.5,
            x_velocity: 4.0,
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
            SKYBLUE,
        );
    }

    fn flap(&mut self) {
        if self.y_velocity > -3.0 {
            self.y_velocity += self.acceleration;
        }
    }

    fn left(&self) -> f32 {
        self.x_displacement
    }

    fn right(&self) -> f32 {
        self.x_displacement + self.width
    }

    fn update(&mut self) -> GameMode {
        self.x_displacement += self.x_velocity;

        // gravity
        if self.y_velocity < 0.5 {
            self.y_velocity += 0.1;
        }

        if self.y_displacement <= 0.0 {
            self.y_displacement = 0.0;
        }
        if self.y_displacement > screen_height() {
            return GameMode::GameOver;
        }
        self.y_displacement += self.y_velocity;

        GameMode::Playing
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

#[derive(Default)]
enum GameMode {
    #[default]
    Menu,
    Playing,
    GameOver,
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

#[macroquad::main("Flipper")]
async fn main() {
    let mut game_state = GameState::default();

    loop {
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
                    ref mut camera,
                    ref finish_line,
                    ref mut flipper,
                    ..
                } = game_state;
                clear_background(DARKBLUE);
                draw_text("Press SPACE to soar", 20.0, 20.0, 30.0, YELLOW);

                if is_key_down(KeyCode::Space) {
                    flipper.flap();
                }
                if is_key_down(KeyCode::Escape) {
                    break;
                }

                camera.update();
                if flipper_finish_line_collision(flipper, finish_line) {
                    game_state.mode = GameMode::Won;
                } else {
                    game_state.mode = flipper.update();
                }

                flipper.draw(camera);
                finish_line.draw(camera);
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
                    game_state.reset();
                    game_state.mode = GameMode::Menu;
                }
            }
            GameMode::Won => {
                clear_background(YELLOW);
                draw_text(
                    "You win! Press SPACE to play again",
                    20.0,
                    20.0,
                    30.0,
                    DARKBLUE,
                );

                if is_key_released(KeyCode::Space) {
                    game_state.reset();
                    game_state.mode = GameMode::Menu;
                }
                if is_key_down(KeyCode::Escape) {
                    break;
                }
            }
        }

        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::{flipper_finish_line_collision, FinishLine, Flipper};

    #[test]
    fn finish_line_left_returns_expected_value() {
        // arrange
        let mut finish_line = FinishLine::default();
        finish_line.with_x_displacement(50.0).with_width(10.0);

        // act
        let result = finish_line.left();

        // assert
        assert!((result - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn finish_line_right_returns_expected_value() {
        // arrange
        let mut finish_line = FinishLine::default();
        finish_line.with_x_displacement(50.0).with_width(10.0);

        // act
        let result = finish_line.right();

        // assert
        assert!((result - 60.0).abs() < f32::EPSILON);
    }

    #[test]
    fn flipper_left_returns_expected_value() {
        // arrange
        let mut flipper = Flipper::default();
        flipper.with_x_displacement(100.0).with_width(30.0);

        // act
        let result = flipper.left();

        // assert
        assert!((result - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn flipper_right_returns_expected_value() {
        // arrange
        let mut flipper = Flipper::default();
        flipper.with_x_displacement(100.0).with_width(30.0);

        // act
        let result = flipper.right();

        // assert
        assert!((result - 130.0).abs() < f32::EPSILON);
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
