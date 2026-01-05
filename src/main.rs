//! Insect Island - A traditional roguelike set on an island of insects.

use crate::utils::loading::*;
use raylib::prelude::*;

mod utils;

/// Entry point of the application.
fn main() {
    // Load config
    let _config = load_config();

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title(_config.get_string("app_name").unwrap().as_str())
        .build();
    let resources = load_resources(&mut rl, &thread);
    let font = resources
        .fonts
        .get("default.ttf")
        .unwrap_or(&resources.fallback_font);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text_ex(
            font,
            "Config Loaded",
            Vector2::new(12.0, 12.0),
            20.0,
            2.0,
            Color::BLACK,
        );
    }
}
