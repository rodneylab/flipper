#![warn(clippy::all, clippy::pedantic)]
use macroquad::color::colors::{DARKBLUE, SKYBLUE, YELLOW};
use macroquad::prelude::*;

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

    fn update(&mut self) {
        // gravity
        if self.velocity < 0.5 {
            self.velocity += 0.1;
        }

        if self.y_displacement < 0.0 {
            self.y_displacement = 0.0;
        } else if self.y_displacement > screen_height() - 20.0 {
            self.y_displacement = screen_height() - 20.0;
        } else {
            self.y_displacement += self.velocity;
        }
    }
}

#[derive(Default)]
struct GameState {
    flipper: Flipper,
}

#[macroquad::main("Flipper")]
async fn main() {
    let mut game_state = GameState::default();

    loop {
        let GameState { ref mut flipper } = game_state;

        if is_key_down(KeyCode::Space) {
            flipper.flap();
        }

        clear_background(DARKBLUE);

        flipper.update();
        flipper.draw();

        draw_text("Press SPACE to soar", 20.0, 20.0, 30.0, YELLOW);

        next_frame().await;
    }
}
