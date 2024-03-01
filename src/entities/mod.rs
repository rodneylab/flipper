use crate::{
    ui::{COLUMBIABLUE, DARKPASTELGREEN, YINMNBLUE},
    GameMode, WINDOW_HEIGHT, WINDOW_WIDTH,
};

use macroquad::{
    audio::{play_sound, PlaySoundParams, Sound},
    shapes::draw_rectangle,
    window::screen_height,
};

pub struct Camera {
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
    pub fn in_view(&self, x_displacement: f32) -> bool {
        self.left_displacement < x_displacement && x_displacement < x_displacement + WINDOW_WIDTH
    }

    pub fn update(&mut self, delta: f32) {
        self.left_displacement += delta * self.pan_speed;
    }
}

pub struct Flipper {
    flap_sound: Option<Sound>,
    x_displacement: f32,
    y_displacement: f32,
    acceleration: f32,
    x_velocity: f32,
    y_velocity: f32,
    width: f32,
    height: f32,
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
            height: 20.0,
        }
    }
}

impl Flipper {
    pub fn draw(&self, camera: &Camera) {
        draw_rectangle(
            self.x_displacement - camera.left_displacement,
            self.y_displacement,
            self.width,
            20.0,
            YINMNBLUE,
        );
    }

    pub fn flap(&mut self) {
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

    pub fn top(&self) -> f32 {
        self.y_displacement
    }

    pub fn bottom(&self) -> f32 {
        self.y_displacement + self.height
    }

    pub fn left(&self) -> f32 {
        self.x_displacement
    }

    pub fn right(&self) -> f32 {
        self.x_displacement + self.width
    }

    pub fn update(&mut self, delta: f32) -> Option<GameMode> {
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

    pub fn with_flap_sound(&mut self, sound: Sound) -> &mut Self {
        self.flap_sound = Some(sound);
        self
    }

    #[allow(dead_code)]
    pub fn with_x_displacement(&mut self, new_x_displacement: f32) -> &mut Self {
        self.x_displacement = new_x_displacement;
        self
    }

    #[allow(dead_code)]
    pub fn with_y_displacement(&mut self, new_y_displacement: f32) -> &mut Self {
        self.y_displacement = new_y_displacement;
        self
    }

    #[allow(dead_code)]
    pub fn with_width(&mut self, new_width: f32) -> &mut Self {
        self.width = new_width;
        self
    }

    #[allow(dead_code)]
    pub fn with_height(&mut self, new_height: f32) -> &mut Self {
        self.height = new_height;
        self
    }
}

pub struct Obstacle {
    gap_length: f32,
    gap_y_displacement: f32,
    width: f32,
    x_displacement: f32,
}

impl Obstacle {
    pub fn new(x_displacement: f32, gap_y_displacement: f32, gap_length: f32) -> Obstacle {
        Obstacle {
            gap_length,
            gap_y_displacement,
            width: 15.0,
            x_displacement,
        }
    }

    pub fn draw(&self, camera: &Camera) {
        if camera.in_view(self.x_displacement) {
            if self.gap_y_displacement > 0.0 {
                draw_rectangle(
                    self.x_displacement - camera.left_displacement,
                    0.0,
                    self.width,
                    self.gap_top(),
                    DARKPASTELGREEN,
                );
            }
            if self.gap_y_displacement > 0.0 {
                let bottom_section_top = self.gap_bottom();
                draw_rectangle(
                    self.x_displacement - camera.left_displacement,
                    bottom_section_top,
                    self.width,
                    WINDOW_HEIGHT - bottom_section_top,
                    DARKPASTELGREEN,
                );
            }
        }
    }

    pub fn gap_top(&self) -> f32 {
        self.gap_y_displacement
    }

    pub fn gap_bottom(&self) -> f32 {
        self.gap_y_displacement + self.gap_length
    }

    pub fn left(&self) -> f32 {
        self.x_displacement
    }

    pub fn right(&self) -> f32 {
        self.x_displacement + self.width
    }

    #[allow(dead_code)]
    pub fn with_x_displacement(&mut self, new_x_displacement: f32) -> &mut Self {
        self.x_displacement = new_x_displacement;
        self
    }

    #[allow(dead_code)]
    pub fn with_width(&mut self, new_width: f32) -> &mut Self {
        self.width = new_width;
        self
    }
}

pub fn flipper_obstruction_collision(flipper: &Flipper, obstruction: &Obstacle) -> bool {
    if flipper.right() < obstruction.left() || flipper.left() > obstruction.right() {
        return false;
    }

    flipper.bottom() > obstruction.gap_bottom() || flipper.top() < obstruction.gap_top()
}

pub struct FinishLine {
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
    pub fn draw(&self, camera: &Camera) {
        if camera.in_view(self.x_displacement) {
            draw_rectangle(
                self.x_displacement - camera.left_displacement,
                0.0,
                self.width,
                WINDOW_HEIGHT,
                COLUMBIABLUE,
            );
        }
    }

    pub fn left(&self) -> f32 {
        self.x_displacement
    }

    pub fn right(&self) -> f32 {
        self.x_displacement + self.width
    }

    #[allow(dead_code)]
    pub fn with_x_displacement(&mut self, new_x_displacement: f32) -> &mut Self {
        self.x_displacement = new_x_displacement;
        self
    }

    #[allow(dead_code)]
    pub fn with_width(&mut self, new_width: f32) -> &mut Self {
        self.width = new_width;
        self
    }
}

pub fn flipper_finish_line_collision(flipper: &Flipper, finish_line: &FinishLine) -> bool {
    flipper.right() > finish_line.left() && flipper.left() < finish_line.right()
}

#[cfg(test)]
mod tests {
    use super::{flipper_finish_line_collision, FinishLine, Flipper, Obstacle};
    use float_cmp::approx_eq;

    #[test]
    fn obstacle_left_returns_expected_value() {
        // arrange
        let mut obstacle = Obstacle::new(1000.0, 200.0, 150.0);
        obstacle.with_width(15.0);

        // act
        let result = obstacle.left();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            1000.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn obstacle_right_returns_expected_value() {
        // arrange
        let mut obstacle = Obstacle::new(1000.0, 200.0, 150.0);
        obstacle.with_width(15.0);

        // act
        let result = obstacle.right();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            1015.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn obstacle_gap_top_returns_expected_value() {
        // arrange
        let gap_top = 200.0_f32;
        let gap_length = 150.0_f32;
        let mut obstacle = Obstacle::new(1000.0, gap_top, gap_length);
        obstacle.with_width(15.0);

        // act
        let result = obstacle.gap_top();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            gap_top,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn obstacle_gap_bottom_returns_expected_value() {
        // arrange
        let gap_top = 200.0_f32;
        let gap_length = 150.0_f32;
        let mut obstacle = Obstacle::new(1000.0, gap_top, gap_length);
        obstacle.with_width(15.0);

        // act
        let result = obstacle.gap_bottom();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            gap_top + gap_length,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

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
    fn flipper_top_returns_expected_value() {
        // arrange
        let mut flipper = Flipper::default();
        flipper
            .with_x_displacement(100.0)
            .with_y_displacement(150.0)
            .with_width(30.0)
            .with_height(50.0);

        // act
        let result = flipper.top();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            150.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn flipper_bottom_returns_expected_value() {
        // arrange
        let mut flipper = Flipper::default();
        flipper
            .with_x_displacement(100.0)
            .with_y_displacement(150.0)
            .with_width(30.0)
            .with_height(50.0);

        // act
        let result = flipper.bottom();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            200.0,
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
