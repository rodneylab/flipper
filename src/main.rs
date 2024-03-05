#![warn(clippy::all, clippy::pedantic)]

mod components;
mod fonts;
mod resources;
mod sound;
mod systems;
mod ui;

use crate::{
    fonts::{load_body_font, load_body_italic_font, load_heading_font},
    resources::{Camera, DeltaTime, GameMode, GameState},
    systems::{
        create_exiting_schedule, create_game_over_schedule, create_menu_schedule,
        create_playing_schedule, create_title_schedule, create_victory_schedule,
        initialise_sound_resources, spawn_entities,
    },
    ui::{
        draw_exit_screen_text, draw_game_over_screen_text, draw_info_text, draw_menu_screen_text,
        draw_title_screen_text, draw_win_screen_text, COLUMBIABLUE, DARKPASTELGREEN, DEEPSKYBLUE,
        MAIZE, YINMNBLUE,
    },
};
use bevy_ecs::{schedule::Schedule, world::World};
use macroquad::{
    input::{is_key_down, prevent_quit, KeyCode},
    logging,
    window::{clear_background, next_frame, Conf},
};

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
async fn main<'a>() {
    prevent_quit();

    let mut world = World::new();
    world.init_resource::<DeltaTime>();
    world.init_resource::<Camera>();
    world.init_resource::<GameState>();

    spawn_entities(&mut world).await;

    let mut initialise_sound_system = Schedule::default();
    initialise_sound_system.add_systems(initialise_sound_resources);
    initialise_sound_system.run(&mut world);

    let heading_font = load_heading_font().await;
    let body_italic_font = load_body_italic_font().await;
    let body_font = load_body_font().await;

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
        logging::trace!("Game mode is {:?}", game_state.mode);

        match &game_state.mode {
            GameMode::Exiting(_resume_mode) => {
                clear_background(MAIZE.into());
                draw_exit_screen_text(&body_font);
                if is_key_down(KeyCode::Enter) {
                    break;
                }
                exiting_schedule.run(&mut world);
            }
            GameMode::Title => {
                clear_background(MAIZE.into());
                draw_title_screen_text(&heading_font, &body_font, &body_italic_font);
                title_schedule.run(&mut world);
            }
            GameMode::Menu => {
                clear_background(DARKPASTELGREEN.into());
                draw_menu_screen_text(&body_font);
                menu_schedule.run(&mut world);
            }
            GameMode::Playing => {
                clear_background(DEEPSKYBLUE.into());
                draw_info_text(&body_font);
                playing_schedule.run(&mut world);
            }
            GameMode::GameOver => {
                clear_background(COLUMBIABLUE.into());
                draw_game_over_screen_text(&body_font);
                game_over_schedule.run(&mut world);
            }
            GameMode::Won => {
                clear_background(YINMNBLUE.into());
                draw_win_screen_text(&body_font);
                victory_schedule.run(&mut world);
            }
        }

        next_frame().await;
    }
}
