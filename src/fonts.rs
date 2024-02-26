use macroquad::text::{load_ttf_font, Font};

pub async fn load_heading_font() -> Font {
    load_ttf_font("./assets/pacifico-v22-latin-regular.ttf")
        .await
        .expect("Unable to load heading font from assets")
}

pub async fn load_body_font() -> Font {
    load_ttf_font("./assets/overpass-v13-latin-regular.ttf")
        .await
        .expect("Unable to load body font from assets")
}

pub async fn load_body_italic_font() -> Font {
    load_ttf_font("./assets/overpass-v13-latin-italic.ttf")
        .await
        .expect("Unable to load body italic font from assets")
}
