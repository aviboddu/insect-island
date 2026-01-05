//! Insect Island - A traditional roguelike set on an island of insects.

use std::time::Duration;

use legion::{Resources, World};
use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::FRect};

/// Entry point of the application.
fn main() {
    println!("Hello World");

    let _world = World::default();
    let _resources = Resources::default();

    // Load minimal configuration and resources for loading game state

    // Initialize SDL2 and subsystems.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Insect Island", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("resources/sprites/BountifulBits_10x10.png")
        .unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    // Run the main game loop.
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        canvas
            .copy_f(
                &texture,
                None,
                FRect::new(0.0, 0.0, 330.0, 460.0).centered_on((400.0, 300.0)),
            )
            .unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
