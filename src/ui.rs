use crate::WINDOW_WIDTH;
use macroquad::{
    color::Color,
    text::{draw_text_ex, measure_text, Font, TextDimensions, TextParams},
};

pub const COLUMBIABLUE: Color = Color {
    r: 189.0 / 255.0,
    g: 213.0 / 255.0,
    b: 234.0 / 255.0,
    a: 1.0,
};

pub const DARKPASTELGREEN: Color = Color {
    r: 76.0 / 255.0,
    g: 185.0 / 255.0,
    b: 68.0 / 255.0,
    a: 1.0,
};

pub const DEEPSKYBLUE: Color = Color {
    r: 0.0,
    g: 187.0 / 255.0,
    b: 249.0 / 255.0,
    a: 1.0,
};

pub const DRABDARKBROWN: Color = Color {
    r: 48.0 / 255.0,
    g: 50.0 / 255.0,
    b: 28.0 / 255.0,
    a: 1.0,
};

pub const MAIZE: Color = Color {
    r: 254.0 / 255.0,
    g: 228.0 / 255.0,
    b: 64.0 / 255.0,
    a: 1.0,
};

pub const YINMNBLUE: Color = Color {
    r: 39.0 / 255.0,
    g: 76.0 / 255.0,
    b: 119.0 / 255.0,
    a: 1.0,
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
            color: YINMNBLUE,
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
            color: YINMNBLUE,
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
            color: YINMNBLUE,
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
            color: YINMNBLUE,
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
            color: YINMNBLUE,
            ..Default::default()
        },
    );
}

pub fn draw_info_text(body_font: &Font) {
    draw_text_ex(
        "Press SPACE to flap your fins.",
        20.0,
        40.0,
        TextParams {
            font_size: 24,
            font: Some(body_font),
            color: DRABDARKBROWN,
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
            color: DRABDARKBROWN,
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
            color: DRABDARKBROWN,
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
            color: YINMNBLUE,
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
            color: YINMNBLUE,
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
            color: YINMNBLUE,
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
            color: COLUMBIABLUE,
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
            color: COLUMBIABLUE,
            ..Default::default()
        },
    );
}
