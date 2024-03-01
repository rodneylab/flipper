use macroquad::audio::{load_sound, Sound};

pub async fn load_background() -> Sound {
    load_sound("./assets/background.wav")
        .await
        .expect("Unable to load background sound")
}

pub async fn load_flap() -> Sound {
    load_sound("./assets/bubbles.wav")
        .await
        .expect("Unable to load flap sound")
}

pub async fn load_game_over() -> Sound {
    load_sound("./assets/game_over.wav")
        .await
        .expect("Unable to load game over sound")
}

pub async fn load_victory() -> Sound {
    load_sound("./assets/victory.wav")
        .await
        .expect("Unable to load victory sound")
}
