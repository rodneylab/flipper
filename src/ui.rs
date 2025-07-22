use crate::{WINDOW_WIDTH, components::Colour};
use macroquad::text::{Font, TextDimensions, TextParams, draw_text_ex, measure_text};

pub const COLUMBIABLUE: Colour = Colour {
    r: 189,
    g: 213,
    b: 234,
    a: 255,
};

pub const DARKPASTELGREEN: Colour = Colour {
    r: 76,
    g: 185,
    b: 68,
    a: 255,
};

pub const DEEPSKYBLUE: Colour = Colour {
    r: 0,
    g: 187,
    b: 249,
    a: 255,
};

pub const DRABDARKBROWN: Colour = Colour {
    r: 48,
    g: 50,
    b: 28,
    a: 255,
};

pub const MAIZE: Colour = Colour {
    r: 254,
    g: 228,
    b: 64,
    a: 255,
};

pub const YINMNBLUE: Colour = Colour {
    r: 39,
    g: 76,
    b: 119,
    a: 255,
};

pub fn draw_exit_screen_text(body_font: &Font) {
    let body_text_0 = "Leave the game?";
    let TextDimensions {
        width: body_text_width_0,
        ..
    } = measure_text(body_text_0, Some(body_font), 36, 1.0);
    draw_text_ex(
        body_text_0,
        0.5 * (WINDOW_WIDTH - body_text_width_0),
        155.0,
        TextParams {
            font_size: 36,
            font: Some(body_font),
            color: YINMNBLUE.into(),
            ..Default::default()
        },
    );

    let body_text_1 = "Press ENTER to confirm,";
    let TextDimensions {
        width: body_text_width_1,
        ..
    } = measure_text(body_text_1, Some(body_font), 36, 1.0);
    draw_text_ex(
        body_text_1,
        0.5 * (WINDOW_WIDTH - body_text_width_1),
        350.0,
        TextParams {
            font_size: 36,
            font: Some(body_font),
            color: YINMNBLUE.into(),
            ..Default::default()
        },
    );

    let body_text_1 = "Esc to go back.";
    let TextDimensions {
        width: body_text_width_1,
        ..
    } = measure_text(body_text_1, Some(body_font), 36, 1.0);
    draw_text_ex(
        body_text_1,
        0.5 * (WINDOW_WIDTH - body_text_width_1),
        425.0,
        TextParams {
            font_size: 36,
            font: Some(body_font),
            color: YINMNBLUE.into(),
            ..Default::default()
        },
    );
}

pub fn draw_game_over_screen_text(body_font: &Font) {
    let body_text_0 = "Game over!";
    let TextDimensions {
        width: body_text_width_0,
        ..
    } = measure_text(body_text_0, Some(body_font), 36, 1.0);
    draw_text_ex(
        body_text_0,
        0.5 * (WINDOW_WIDTH - body_text_width_0),
        155.0,
        TextParams {
            font_size: 36,
            font: Some(body_font),
            color: YINMNBLUE.into(),
            ..Default::default()
        },
    );

    let body_text_1 = "Press SPACE to play again.";
    let TextDimensions {
        width: body_text_width_1,
        ..
    } = measure_text(body_text_1, Some(body_font), 48, 1.0);
    draw_text_ex(
        body_text_1,
        0.5 * (WINDOW_WIDTH - body_text_width_1),
        300.0,
        TextParams {
            font_size: 48,
            font: Some(body_font),
            color: YINMNBLUE.into(),
            ..Default::default()
        },
    );
}

pub fn draw_info_text(score: u32, body_font: &Font) {
    let info_text = if score == 0 {
        String::from("Press SPACE to flap your fins.")
    } else {
        format!("Score: {score}")
    };
    draw_text_ex(
        &info_text,
        20.0,
        40.0,
        TextParams {
            font_size: 24,
            font: Some(body_font),
            color: DRABDARKBROWN.into(),
            ..Default::default()
        },
    );
}

pub fn draw_menu_screen_text(body_font: &Font) {
    let body_text_0 = "Are you ready?";
    let TextDimensions {
        width: body_text_width_0,
        ..
    } = measure_text(body_text_0, Some(body_font), 36, 1.0);
    draw_text_ex(
        body_text_0,
        0.5 * (WINDOW_WIDTH - body_text_width_0),
        175.0,
        TextParams {
            font_size: 36,
            font: Some(body_font),
            color: DRABDARKBROWN.into(),
            ..Default::default()
        },
    );

    let body_text_1 = "Press SPACE to play.";
    let TextDimensions {
        width: body_text_width_1,
        ..
    } = measure_text(body_text_1, Some(body_font), 48, 1.0);
    draw_text_ex(
        body_text_1,
        0.5 * (WINDOW_WIDTH - body_text_width_1),
        350.0,
        TextParams {
            font_size: 48,
            font: Some(body_font),
            color: DRABDARKBROWN.into(),
            ..Default::default()
        },
    );
}

pub fn draw_title_screen_text(heading_font: &Font, body_font: &Font, body_italic_font: &Font) {
    let heading_text = "Flipper";
    let TextDimensions {
        width: heading_width,
        ..
    } = measure_text(heading_text, Some(heading_font), 144, 1.0);
    draw_text_ex(
        heading_text,
        0.5 * (WINDOW_WIDTH - heading_width),
        250.0,
        TextParams {
            font_size: 144,
            font: Some(heading_font),
            color: YINMNBLUE.into(),
            rotation: -0.06,
            ..Default::default()
        },
    );

    let subheading_text = "the gentle giant manta ray";
    let TextDimensions {
        width: subheading_width,
        ..
    } = measure_text(subheading_text, Some(body_italic_font), 36, 1.0);
    draw_text_ex(
        subheading_text,
        0.5 * (WINDOW_WIDTH - subheading_width),
        400.0,
        TextParams {
            font_size: 36,
            font: Some(body_italic_font),
            color: YINMNBLUE.into(),
            ..Default::default()
        },
    );

    let body_text = "game loading...";
    let TextDimensions {
        width: body_text_width,
        ..
    } = measure_text(body_text, Some(body_font), 28, 1.0);
    draw_text_ex(
        body_text,
        0.5 * (WINDOW_WIDTH - body_text_width),
        550.0,
        TextParams {
            font_size: 28,
            font: Some(body_font),
            color: YINMNBLUE.into(),
            ..Default::default()
        },
    );
}

pub fn draw_win_screen_text(body_font: &Font) {
    let body_text_0 = "*** You won! ***";
    let TextDimensions {
        width: body_text_width_0,
        ..
    } = measure_text(body_text_0, Some(body_font), 72, 1.0);
    draw_text_ex(
        body_text_0,
        0.5 * (WINDOW_WIDTH - body_text_width_0),
        155.0,
        TextParams {
            font_size: 72,
            font: Some(body_font),
            color: COLUMBIABLUE.into(),
            ..Default::default()
        },
    );

    let body_text_1 = "Press SPACE to play again.";
    let TextDimensions {
        width: body_text_width_1,
        ..
    } = measure_text(body_text_1, Some(body_font), 36, 1.0);
    draw_text_ex(
        body_text_1,
        0.5 * (WINDOW_WIDTH - body_text_width_1),
        350.0,
        TextParams {
            font_size: 36,
            font: Some(body_font),
            color: COLUMBIABLUE.into(),
            ..Default::default()
        },
    );
}
