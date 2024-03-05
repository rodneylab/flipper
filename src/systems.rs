use bevy_ecs::{
    schedule::Schedule,
    system::{Query, Res, ResMut},
    world::World,
};

use crate::{
    components::{Colour, FinishLine, Flipper, ObstacleShape, Position, RectangleShape, Velocity},
    resources::{Camera, GameMode, GameState, ResumeGameMode},
    sound::{
        load_background as load_background_sound, load_flap as load_flap_sound,
        load_game_over as load_game_over_sound, load_victory as load_victory_sound,
        play_sound_once, start_playing_looped as start_playing_looped_sound,
        stop_playing_looped as stop_playing_looped_sound,
    },
    ui::{COLUMBIABLUE, DARKPASTELGREEN, YINMNBLUE},
    DeltaTime, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use futures::executor::block_on;
use macroquad::{
    audio::{play_sound, PlaySoundParams, Sound},
    input::{
        is_key_down, is_key_released, is_mouse_button_pressed, is_quit_requested, KeyCode,
        MouseButton,
    },
    logging::{self, info},
    shapes::draw_rectangle,
    time::{get_frame_time, get_time},
};

pub async fn spawn_entities(world: &mut World) {
    let _flipper_entity = world
        .spawn((
            Flipper {
                flap_sound: Some(load_flap_sound().await),
            },
            RectangleShape {
                width: 20.0,
                height: 20.0,
            },
            YINMNBLUE,
            Position {
                x: 20.0,
                y: 0.5 * WINDOW_HEIGHT - 10.0,
            },
            Velocity { x: 240.0, y: 0.0 },
        ))
        .id();
    let _finish_line_entity = world
        .spawn((
            FinishLine,
            RectangleShape {
                width: 5.0,
                height: WINDOW_HEIGHT,
            },
            COLUMBIABLUE,
            Position {
                x: 4.0 * WINDOW_WIDTH,
                y: 0.0,
            },
        ))
        .id();
    world.spawn((
        Position { x: 600.0, y: 0.0 },
        DARKPASTELGREEN,
        ObstacleShape {
            gap_length: 100.0,
            gap_y_displacement: 300.0,
            width: 15.0,
        },
    ));
    world.spawn((
        Position { x: 900.0, y: 0.0 },
        DARKPASTELGREEN,
        ObstacleShape {
            gap_length: 75.0,
            gap_y_displacement: 350.0,
            width: 15.0,
        },
    ));
    world.spawn((
        Position { x: 1200.0, y: 0.0 },
        DARKPASTELGREEN,
        ObstacleShape {
            gap_length: 75.0,
            gap_y_displacement: 300.0,
            width: 15.0,
        },
    ));
    world.spawn((
        Position { x: 1500.0, y: 0.0 },
        DARKPASTELGREEN,
        ObstacleShape {
            gap_length: 75.0,
            gap_y_displacement: 300.0,
            width: 15.0,
        },
    ));
}

pub async fn initialise_sound_resources_async(game_state: &mut GameState) {
    let background_sound = load_background_sound().await;
    game_state.background_sound = Some(background_sound);

    let victory_sound = load_victory_sound().await;
    game_state.victory_sound = Some(victory_sound);

    let game_over_sound = load_game_over_sound().await;
    game_state.game_over_sound = Some(game_over_sound);
}

pub fn initialise_sound_resources(mut game_state: ResMut<GameState>) {
    block_on(initialise_sound_resources_async(&mut game_state));
}

pub fn update_delta_time(mut delta_time: ResMut<DeltaTime>) {
    delta_time.seconds = get_frame_time();
}

#[allow(clippy::needless_pass_by_value)]
pub fn draw_rectangles(query: Query<(&Position, &RectangleShape, &Colour)>, camera: Res<Camera>) {
    for (position, shape, colour) in query.iter() {
        let Position { x, y } = position;
        let RectangleShape { width, height } = shape;
        if camera.in_view(*x) {
            draw_rectangle(
                *x - camera.left_displacement,
                *y,
                *width,
                *height,
                (*colour).into(),
            );
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn draw_obstacles(query: Query<(&Position, &ObstacleShape, &Colour)>, camera: Res<Camera>) {
    for (position, shape, colour) in query.iter() {
        let Position { x, .. } = position;
        if camera.in_view(*x) {
            if shape.gap_y_displacement > 0.0 {
                draw_rectangle(
                    *x - camera.left_displacement,
                    0.0,
                    shape.width,
                    shape.gap_top(),
                    (*colour).into(),
                );
            }
            let bottom_section_top = shape.gap_bottom();
            if bottom_section_top < WINDOW_HEIGHT {
                draw_rectangle(
                    *x - camera.left_displacement,
                    bottom_section_top,
                    shape.width,
                    WINDOW_HEIGHT - bottom_section_top,
                    (*colour).into(),
                );
            }
        }
    }
}

fn handle_before_transisition_to_won(
    background_sound: Option<&Sound>,
    victory_sound: Option<&Sound>,
) {
    if let Some(value) = background_sound {
        stop_playing_looped_sound(value);
    }
    if let Some(value) = victory_sound {
        play_sound_once(value);
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn handle_flipper_finish_line_collision(
    flipper_query: Query<(&Flipper, &Position, &RectangleShape)>,
    finish_line_query: Query<(&FinishLine, &Position, &RectangleShape)>,
    mut game_state: ResMut<GameState>,
) {
    for (_flipper, flipper_position, flipper_shape) in &flipper_query {
        for (_finish_line, finish_line_position, finish_line_shape) in finish_line_query.iter() {
            if flipper_shape.right(flipper_position) > finish_line_shape.left(finish_line_position)
                && flipper_shape.left(flipper_position)
                    < finish_line_shape.right(finish_line_position)
            {
                game_state.mode = GameMode::Won;
                handle_before_transisition_to_won(
                    game_state.background_sound.as_ref(),
                    game_state.victory_sound.as_ref(),
                );
            }
        }
    }
}

pub fn handle_exit(mut game_state: ResMut<'_, GameState>) {
    if is_key_released(KeyCode::Escape) {
        if let GameMode::Exiting(value) = &game_state.mode {
            match value {
                ResumeGameMode::Playing => game_state.mode = GameMode::Playing,
                ResumeGameMode::Menu => game_state.mode = GameMode::Menu,
            }
        } else {
            logging::error!("Handle exit called from outside GameMode::Exiting");
            game_state.mode = GameMode::Menu;
        }
    }
}

fn handle_before_transisition_to_game_over(
    background_sound: Option<&Sound>,
    game_over_sound: Option<&Sound>,
) {
    if let Some(value) = background_sound {
        stop_playing_looped_sound(value);
    }
    if let Some(value) = game_over_sound {
        play_sound_once(value);
    }
}

fn obstacle_flipper_collision(
    obstacle_shape: &ObstacleShape,
    obstacle_position: &Position,
    flipper_shape: &RectangleShape,
    flipper_position: &Position,
) -> bool {
    let horizontal_overlap = flipper_shape.right(flipper_position)
        > obstacle_shape.left(obstacle_position)
        && flipper_shape.left(flipper_position) < obstacle_shape.right(obstacle_position);

    if !horizontal_overlap {
        return false;
    }

    // test for vertical overlap
    flipper_shape.bottom(flipper_position) > obstacle_shape.gap_bottom()
        || flipper_shape.top(flipper_position) < obstacle_shape.gap_top()
}

#[allow(clippy::needless_pass_by_value)]
pub fn handle_obstacle_flipper_collision(
    obstacle_query: Query<'_, '_, (&Position, &ObstacleShape)>,
    flipper_query: Query<'_, '_, (&Flipper, &Position, &RectangleShape)>,
    mut game_state: ResMut<'_, GameState>,
) {
    for (_flipper, flipper_position, flipper_shape) in &flipper_query {
        for (obstacle_position, obstacle_shape) in obstacle_query.iter() {
            if obstacle_flipper_collision(
                obstacle_shape,
                obstacle_position,
                flipper_shape,
                flipper_position,
            ) {
                info!("Game Over triggered: Flipper-Obstacle collision");
                handle_before_transisition_to_game_over(
                    game_state.background_sound.as_ref(),
                    game_state.game_over_sound.as_ref(),
                );
                game_state.mode = GameMode::GameOver;
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_camera(delta_time: Res<DeltaTime>, mut camera: ResMut<Camera>) {
    camera.update(delta_time.seconds);
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_positions(mut query: Query<(&mut Position, &Velocity)>, delta_time: Res<DeltaTime>) {
    for (mut position, velocity) in &mut query {
        position.x += delta_time.seconds * velocity.x;
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_flipper_vertical_position(
    mut query: Query<(&Flipper, &mut Position, &mut Velocity)>,
    delta_time: Res<DeltaTime>,
    mut game_state: ResMut<GameState>,
) {
    for (_flipper, mut position, mut velocity) in &mut query {
        if velocity.y < 30.0 {
            velocity.y += 6.0;
        }

        if position.y <= 0.0 {
            position.y = 0.0;
        } else if position.y > WINDOW_HEIGHT {
            println!("Game over!");
            game_state.mode = GameMode::GameOver;
        }
        position.y += delta_time.seconds * velocity.y;
    }
}

pub fn handle_flipper_controls(mut query: Query<(&Flipper, &mut Velocity)>) {
    for (flipper, mut velocity) in &mut query {
        let Flipper { flap_sound } = flipper;
        if is_key_down(KeyCode::Space) {
            if let Some(value) = flap_sound {
                play_sound(
                    value,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.05,
                    },
                );
            };
            if velocity.y > -180.0 {
                velocity.y += -30.0;
            }
        }
    }
}

pub fn handle_replay(
    mut query: Query<(&Flipper, &mut Position)>,
    mut camera: ResMut<Camera>,
    mut game_state: ResMut<GameState>,
) {
    if is_key_released(KeyCode::Space) {
        //reset camera position
        camera.left_displacement = 0.0;

        // reset flipper position
        for (_flipper, mut position) in &mut query {
            position.x = 20.0;
            position.y = 0.5 * WINDOW_HEIGHT - 10.0;
        }

        // update game mode
        game_state.mode = GameMode::Menu;
    }
}

pub fn handle_skip_title(mut game_state: ResMut<GameState>) {
    if is_key_released(KeyCode::Space)
        || is_mouse_button_pressed(MouseButton::Left)
        || get_time() > 5.0
    {
        game_state.mode = GameMode::Menu;
    }
}
pub fn handle_request_quit(mut game_state: ResMut<GameState>) {
    if is_key_released(KeyCode::Escape) || is_quit_requested() {
        game_state.mode = match &game_state.mode {
            GameMode::Playing => GameMode::Exiting(ResumeGameMode::Playing),
            GameMode::GameOver | GameMode::Menu | GameMode::Title | GameMode::Won => {
                GameMode::Exiting(ResumeGameMode::Menu)
            }
            GameMode::Exiting(_) => {
                logging::error!("Unexpected call to handle request quit from exiting state.");
                GameMode::Menu
            }
        }
    }
}

pub fn handle_start_game(mut game_state: ResMut<GameState>) {
    if is_key_down(KeyCode::Space) {
        if let Some(value) = &game_state.background_sound {
            start_playing_looped_sound(value);
        }
        game_state.mode = GameMode::Playing;
    }
}

pub fn create_playing_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(handle_request_quit)
        .add_systems(update_delta_time)
        .add_systems(update_camera)
        .add_systems(draw_rectangles)
        .add_systems(draw_obstacles)
        .add_systems(handle_flipper_controls)
        .add_systems(update_flipper_vertical_position)
        .add_systems(handle_obstacle_flipper_collision)
        .add_systems(handle_flipper_finish_line_collision)
        .add_systems(update_positions);

    result
}

pub fn create_exiting_schedule() -> Schedule {
    let mut result = Schedule::default();
    result.add_systems(handle_exit);

    result
}

pub fn create_title_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(handle_skip_title)
        .add_systems(handle_request_quit);
    result
}

pub fn create_menu_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(handle_start_game)
        .add_systems(handle_request_quit);

    result
}

pub fn create_victory_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(handle_request_quit)
        .add_systems(handle_replay);

    result
}

pub fn create_game_over_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(handle_request_quit)
        .add_systems(handle_replay);

    result
}

#[cfg(test)]
mod tests {
    use super::{obstacle_flipper_collision, ObstacleShape, Position, RectangleShape};

    #[test]
    fn flipper_obstacle_collision_successfully_reports_miss() {
        // arrange
        let obstacle_shape = ObstacleShape {
            gap_length: 200.0,
            gap_y_displacement: 150.0,
            width: 15.0,
        };
        let obstacle_position = Position { x: 500.0, y: 0.0 };
        let flipper_shape = RectangleShape {
            width: 20.0,
            height: 20.0,
        };
        let flipper_position = Position { x: 100.0, y: 120.0 };

        // act
        let result = obstacle_flipper_collision(
            &obstacle_shape,
            &obstacle_position,
            &flipper_shape,
            &flipper_position,
        );

        // assert
        assert!(!result);
    }

    #[test]
    fn flipper_obstacle_collision_successfully_reports_collision() {
        // arrange
        let obstacle_shape = ObstacleShape {
            gap_length: 200.0,
            gap_y_displacement: 150.0,
            width: 15.0,
        };
        let obstacle_position = Position { x: 90.0, y: 0.0 };
        let flipper_shape = RectangleShape {
            width: 20.0,
            height: 20.0,
        };
        let flipper_position = Position { x: 100.0, y: 120.0 };

        // act
        let result = obstacle_flipper_collision(
            &obstacle_shape,
            &obstacle_position,
            &flipper_shape,
            &flipper_position,
        );

        // assert
        assert!(result);
    }
}
