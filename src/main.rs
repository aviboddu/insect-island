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

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Config Loaded", 12, 12, 20, Color::BLACK);
    }
}
