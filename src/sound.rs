use macroquad::audio::{PlaySoundParams, Sound, play_sound, stop_sound};

pub fn play_sound_once(sound: &Sound) {
    play_sound(
        sound,
        PlaySoundParams {
            looped: false,
            volume: 0.05,
        },
    );
}

pub fn start_playing_looped(sound: &Sound) {
    play_sound(
        sound,
        PlaySoundParams {
            looped: true,
            volume: 0.05,
        },
    );
}

pub fn stop_playing_looped(sound: &Sound) {
    stop_sound(sound);
}
