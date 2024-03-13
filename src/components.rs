use bevy_ecs::component::Component;

#[derive(Clone, Component, Copy, Debug, Default)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Colour> for macroquad::color::Color {
    fn from(colour: Colour) -> Self {
        let Colour { r, g, b, a } = colour;
        macroquad::color::Color {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
            a: f32::from(a) / 255.0,
        }
    }
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Colour { r, g, b, a }
    }
}

#[derive(Component)]
pub struct FinishLine;

#[derive(Component, Debug, Default)]
pub struct Flipper {}

#[derive(Component, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Default)]
pub struct RectangleShape {
    pub width: f32,
    pub height: f32,
}

impl RectangleShape {
    #[allow(clippy::unused_self)]
    pub fn left(&self, position: &Position) -> f32 {
        position.x
    }

    pub fn right(&self, position: &Position) -> f32 {
        position.x + self.width
    }

    #[allow(clippy::unused_self)]
    pub fn top(&self, position: &Position) -> f32 {
        position.y
    }

    pub fn bottom(&self, position: &Position) -> f32 {
        position.y + self.height
    }
}

#[derive(Component, Debug, Default)]
pub struct ObstacleShape {
    pub gap_length: f32,
    pub gap_y_displacement: f32,
    pub width: f32,
}

impl ObstacleShape {
    #[allow(clippy::unused_self)]
    pub fn left(&self, position: &Position) -> f32 {
        position.x
    }

    pub fn right(&self, position: &Position) -> f32 {
        position.x + self.width
    }

    pub fn gap_top(&self) -> f32 {
        self.gap_y_displacement
    }

    pub fn gap_bottom(&self) -> f32 {
        self.gap_y_displacement + self.gap_length
    }
}

#[derive(Component, Debug, Default)]
pub struct Score {
    pub value: u32,
}

#[cfg(test)]
mod tests {
    use super::{ObstacleShape, Position, RectangleShape};
    use float_cmp::approx_eq;

    #[test]
    fn obstacle_shape_left_returns_expected_value() {
        // arrange
        let shape = ObstacleShape {
            gap_length: 200.0,
            gap_y_displacement: 150.0,
            width: 15.0,
        };

        // act
        let result = shape.left(&Position { x: 1000.0, y: 0.0 });

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
        let shape = ObstacleShape {
            gap_length: 200.0,
            gap_y_displacement: 150.0,
            width: 15.0,
        };

        // act
        let result = shape.right(&Position { x: 1000.0, y: 0.0 });

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
        let shape = ObstacleShape {
            gap_length: 150.0,
            gap_y_displacement: 200.0,
            width: 15.0,
        };

        // act
        let result = shape.gap_top();

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
    fn obstacle_gap_bottom_returns_expected_value() {
        // arrange
        let shape = ObstacleShape {
            gap_length: 150.0,
            gap_y_displacement: 200.0,
            width: 15.0,
        };

        // act
        let result = shape.gap_bottom();

        // assert
        assert!(approx_eq!(
            f32,
            result,
            350.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn rectangle_shape_left_returns_expected_value() {
        // arrange
        let shape = RectangleShape {
            width: 10.0,
            height: 600.0,
        };
        let position = Position { x: 1000.0, y: 0.0 };

        // act
        let result = shape.left(&position);

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
    fn rectangle_shape_right_returns_expected_value() {
        // arrange
        let shape = RectangleShape {
            width: 10.0,
            height: 600.0,
        };
        let position = Position { x: 1000.0, y: 0.0 };

        // act
        let result = shape.right(&position);

        // assert
        assert!(approx_eq!(
            f32,
            result,
            1010.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn rectangle_shape_top_returns_expected_value() {
        // arrange
        let shape = RectangleShape {
            width: 10.0,
            height: 600.0,
        };
        let position = Position {
            x: 1000.0,
            y: 500.0,
        };

        // act
        let result = shape.top(&position);

        // assert
        assert!(approx_eq!(
            f32,
            result,
            500.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }

    #[test]
    fn rectangle_shape_bottom_returns_expected_value() {
        // arrange
        let shape = RectangleShape {
            width: 10.0,
            height: 100.0,
        };
        let position = Position {
            x: 1000.0,
            y: 500.0,
        };

        // act
        let result = shape.bottom(&position);

        // assert
        assert!(approx_eq!(
            f32,
            result,
            600.0,
            epsilon = f32::EPSILON,
            ulps = 2
        ));
    }
}
