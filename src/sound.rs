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
