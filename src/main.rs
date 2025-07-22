#![warn(clippy::all, clippy::pedantic)]

mod asset_manager;
mod components;
mod resources;
mod sound;
mod systems;
mod ui;

use crate::{
    resources::{Camera, DeltaTime, GameMode, GameState},
    systems::{
        create_exiting_schedule, create_game_over_schedule, create_menu_schedule,
        create_playing_schedule, create_title_schedule, create_victory_schedule, initialise_fonts,
        initialise_sound_resources, spawn_entities,
    },
    ui::{COLUMBIABLUE, DARKPASTELGREEN, DEEPSKYBLUE, MAIZE, YINMNBLUE},
};
use bevy_ecs::{schedule::Schedule, world::World};
use macroquad::{
    input::{KeyCode, is_key_down, prevent_quit},
    window::{Conf, clear_background, next_frame},
};
use resources::{ClearedObstacles, GameAssets};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn conf() -> Conf {
    #[allow(clippy::cast_possible_truncation)]
    Conf {
        window_title: String::from("Flipper"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    prevent_quit();

    let mut world = World::new();
    world.init_resource::<DeltaTime>();
    world.init_resource::<Camera>();
    world.init_resource::<GameState>();
    world.init_resource::<GameAssets>();
    world.init_resource::<ClearedObstacles>();

    spawn_entities(&mut world).await;

    let mut initialise_sound_system = Schedule::default();
    initialise_sound_system.add_systems(initialise_sound_resources);
    initialise_sound_system.run(&mut world);

    let mut initialise_fonts_system = Schedule::default();
    initialise_fonts_system.add_systems(initialise_fonts);
    initialise_fonts_system.run(&mut world);

    let mut exiting_schedule = create_exiting_schedule();
    let mut title_schedule = create_title_schedule();
    let mut menu_schedule = create_menu_schedule();
    let mut playing_schedule = create_playing_schedule();
    let mut victory_schedule = create_victory_schedule();
    let mut game_over_schedule = create_game_over_schedule();

    loop {
        let game_state = world
            .get_resource::<GameState>()
            .expect("Expected state to have been initialised.");
        //logging::trace!("Game mode is {:?}", game_state.mode);

        match &game_state.mode {
            GameMode::Exiting(_resume_mode) => {
                clear_background(MAIZE.into());
                if is_key_down(KeyCode::Enter) {
                    break;
                }
                exiting_schedule.run(&mut world);
            }
            GameMode::Title => {
                clear_background(MAIZE.into());
                title_schedule.run(&mut world);
            }
            GameMode::Menu => {
                clear_background(DARKPASTELGREEN.into());
                menu_schedule.run(&mut world);
            }
            GameMode::Playing => {
                clear_background(DEEPSKYBLUE.into());
                playing_schedule.run(&mut world);
            }
            GameMode::GameOver => {
                clear_background(COLUMBIABLUE.into());
                game_over_schedule.run(&mut world);
            }
            GameMode::Won => {
                clear_background(YINMNBLUE.into());
                victory_schedule.run(&mut world);
            }
        }

        next_frame().await;
    }
}
