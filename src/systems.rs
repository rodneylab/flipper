use crate::{
    asset_manager::AssetManager,
    components::{
        Colour, FinishLine, Flipper, ObstacleShape, Position, RectangleShape, Score, Velocity,
    },
    resources::{
        Camera, ClearedObstacles, GameAssets, GameFonts, GameMode, GameState, ResumeGameMode,
    },
    sound::{
        play_sound_once, start_playing_looped as start_playing_looped_sound,
        stop_playing_looped as stop_playing_looped_sound,
    },
    ui::{
        draw_exit_screen_text, draw_game_over_screen_text, draw_info_text, draw_menu_screen_text,
        draw_title_screen_text, draw_win_screen_text, COLUMBIABLUE, DARKPASTELGREEN, YINMNBLUE,
    },
    DeltaTime, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use bevy_ecs::{
    entity::Entity,
    query::With,
    schedule::Schedule,
    system::{Query, Res, ResMut},
    world::World,
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
            Flipper {},
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
            Score { value: 0 },
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

pub fn initialise_fonts(mut game_assets: ResMut<GameAssets>) {
    let game_fonts = &mut game_assets.fonts;
    block_on(AssetManager::load_fonts(game_fonts));
}

pub fn initialise_sound_resources(game_state: ResMut<GameAssets>) {
    let game_sounds = &mut game_state.into_inner().sounds;
    block_on(AssetManager::load_sounds(game_sounds));
}

pub fn update_delta_time(mut delta_time: ResMut<DeltaTime>) {
    delta_time.seconds = get_frame_time();
}

#[allow(clippy::needless_pass_by_value)]
fn draw_rectangles(query: Query<(&Position, &RectangleShape, &Colour)>, camera: Res<Camera>) {
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
fn draw_obstacles(query: Query<(&Position, &ObstacleShape, &Colour)>, camera: Res<Camera>) {
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
fn handle_flipper_finish_line_collision(
    flipper_query: Query<(&Position, &RectangleShape), With<Flipper>>,
    finish_line_query: Query<(&FinishLine, &Position, &RectangleShape)>,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<GameState>,
) {
    let (flipper_position, flipper_shape) = flipper_query.single();
    for (_finish_line, finish_line_position, finish_line_shape) in finish_line_query.iter() {
        if flipper_shape.right(flipper_position) > finish_line_shape.left(finish_line_position)
            && flipper_shape.left(flipper_position) < finish_line_shape.right(finish_line_position)
        {
            game_state.mode = GameMode::Won;
            handle_before_transisition_to_won(
                game_assets.sounds.background.as_ref(),
                game_assets.sounds.victory.as_ref(),
            );
        }
    }
}

fn handle_exit(mut game_state: ResMut<'_, GameState>) {
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

#[derive(Debug, PartialEq)]
pub enum FlipperObstacleCollision {
    Gap,
    Obstacle,
}

fn obstacle_flipper_collision(
    obstacle_shape: &ObstacleShape,
    obstacle_position: &Position,
    flipper_shape: &RectangleShape,
    flipper_position: &Position,
) -> Option<FlipperObstacleCollision> {
    let horizontal_overlap = flipper_shape.right(flipper_position)
        > obstacle_shape.left(obstacle_position)
        && flipper_shape.left(flipper_position) < obstacle_shape.right(obstacle_position);

    if !horizontal_overlap {
        return None;
    }

    if flipper_shape.bottom(flipper_position) < obstacle_shape.gap_bottom()
        && flipper_shape.top(flipper_position) > obstacle_shape.gap_top()
    {
        Some(FlipperObstacleCollision::Gap)
    } else {
        Some(FlipperObstacleCollision::Obstacle)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_obstacle_flipper_collision(
    obstacle_query: Query<'_, '_, (Entity, &Position, &ObstacleShape)>,
    mut flipper_query: Query<(&mut Score, &Position, &RectangleShape), With<Flipper>>,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<GameState>,
    mut cleared_obstacles: ResMut<ClearedObstacles>,
) {
    let (mut score, flipper_position, flipper_shape) = flipper_query.single_mut();
    for (entity, obstacle_position, obstacle_shape) in obstacle_query.iter() {
        if !cleared_obstacles.obstacles.iter().any(|val| *val == entity) {
            if let Some(value) = obstacle_flipper_collision(
                obstacle_shape,
                obstacle_position,
                flipper_shape,
                flipper_position,
            ) {
                match value {
                    FlipperObstacleCollision::Obstacle => {
                        info!("Game Over triggered: Flipper-Obstacle collision");
                        handle_before_transisition_to_game_over(
                            game_assets.sounds.background.as_ref(),
                            game_assets.sounds.game_over.as_ref(),
                        );
                        game_state.mode = GameMode::GameOver;
                    }
                    FlipperObstacleCollision::Gap => {
                        info!("Obstacle cleared");
                        score.value += 1;
                        cleared_obstacles.obstacles.push(entity);
                        if let Some(value) = &game_assets.sounds.obstacle_cleared {
                            play_sound_once(value);
                        };
                    }
                }
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
    mut query: Query<(&mut Position, &mut Velocity), With<Flipper>>,
    delta_time: Res<DeltaTime>,
    mut game_state: ResMut<GameState>,
) {
    let (mut position, mut velocity) = query.single_mut();
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

#[allow(clippy::needless_pass_by_value)]
pub fn handle_flipper_controls(
    mut query: Query<&mut Velocity, With<Flipper>>,
    game_assets: Res<GameAssets>,
) {
    //for mut velocity in &mut query {
    let mut velocity = query.single_mut();
    if is_key_down(KeyCode::Space) {
        if let Some(value) = &game_assets.sounds.flap {
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
    // }
}

pub fn handle_replay(
    mut query: Query<(&mut Score, &mut Position), With<Flipper>>,
    mut camera: ResMut<Camera>,
    mut cleared_obstacles: ResMut<ClearedObstacles>,
    mut game_state: ResMut<GameState>,
) {
    if is_key_released(KeyCode::Space) {
        //reset camera position
        camera.left_displacement = 0.0;

        // reset flipper position and score
        let (mut score, mut position) = query.single_mut();
        score.value = 0;
        position.x = 20.0;
        position.y = 0.5 * WINDOW_HEIGHT - 10.0;

        // reset cleared obstales
        cleared_obstacles.obstacles = Vec::new();

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

#[allow(clippy::needless_pass_by_value)]
pub fn handle_start_game(game_assets: Res<GameAssets>, mut game_state: ResMut<GameState>) {
    if is_key_down(KeyCode::Space) {
        if let Some(value) = &game_assets.sounds.background {
            start_playing_looped_sound(value);
        }
        game_state.mode = GameMode::Playing;
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_game_over_ui(game_assets: Res<GameAssets>) {
    let GameAssets {
        fonts: GameFonts {
            body: body_font, ..
        },
        ..
    } = game_assets.into_inner();
    if let Some(body_font_value) = body_font {
        draw_game_over_screen_text(body_font_value);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_menu_ui(game_assets: Res<GameAssets>) {
    let GameAssets {
        fonts: GameFonts {
            body: body_font, ..
        },
        ..
    } = game_assets.into_inner();
    if let Some(body_font_value) = body_font {
        draw_menu_screen_text(body_font_value);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_title_ui(game_assets: Res<GameAssets>) {
    let GameAssets {
        fonts:
            GameFonts {
                body: body_font,
                body_italic: body_italic_font,
                heading: heading_font,
                ..
            },
        ..
    } = game_assets.into_inner();
    if let (Some(body_font_value), Some(body_italic_font_value), Some(heading_font_value)) =
        (body_font, body_italic_font, heading_font)
    {
        draw_title_screen_text(heading_font_value, body_font_value, body_italic_font_value);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_exit_ui(game_assets: Res<GameAssets>) {
    let GameAssets {
        fonts: GameFonts {
            body: body_font, ..
        },
        ..
    } = game_assets.into_inner();
    if let Some(value) = body_font {
        draw_exit_screen_text(value);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_ui(query: Query<&Score, With<Flipper>>, game_assets: Res<GameAssets>) {
    let score = query.single().value;
    let assets = game_assets.into_inner();

    let GameAssets {
        fonts: GameFonts {
            body: body_font, ..
        },
        ..
    } = assets;
    if let Some(value) = body_font {
        draw_info_text(score, value);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_win_screen_ui(game_assets: Res<GameAssets>) {
    let GameAssets {
        fonts: GameFonts {
            body: body_font, ..
        },
        ..
    } = game_assets.into_inner();
    if let Some(value) = body_font {
        draw_win_screen_text(value);
    }
}
pub fn create_playing_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(update_ui)
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
    result.add_systems(update_exit_ui).add_systems(handle_exit);

    result
}

pub fn create_title_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(update_title_ui)
        .add_systems(handle_skip_title)
        .add_systems(handle_request_quit);
    result
}

pub fn create_menu_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(update_menu_ui)
        .add_systems(handle_start_game)
        .add_systems(handle_request_quit);

    result
}

pub fn create_victory_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(update_win_screen_ui)
        .add_systems(handle_request_quit)
        .add_systems(handle_replay);

    result
}

pub fn create_game_over_schedule() -> Schedule {
    let mut result = Schedule::default();
    result
        .add_systems(update_game_over_ui)
        .add_systems(handle_request_quit)
        .add_systems(handle_replay);

    result
}

#[cfg(test)]
mod tests {
    use crate::systems::FlipperObstacleCollision;

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
        assert_eq!(result, None);
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
        assert_eq!(result, Some(FlipperObstacleCollision::Obstacle));
    }
}
