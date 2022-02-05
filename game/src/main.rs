// demon attack with freeze functions in rust using ggez

// imports
extern crate ggez;
use ggez::{
    conf,
    event,
    graphics,
    Context,
    GameResult,
};

// window setup
pub struct WindowSetup {
    pub width: f32,
    pub height: f32,
    pub title: String,
}

fn main() {
    // create context
    let (mut ctx, mut event_loop) = conf::Conf::new()
        .window_setup(conf::WindowSetup::default().title("Freeze Attack"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()
        .expect("Could not create ggez context");

    // Set window position here
    let (ref mut ctx, event_loop) = &mut cb.build()?; 
    let window = graphics::window(ctx);
    let mut pos = window.get_position().unwrap();
    pos.x = 0.0;
    pos.y = 0.0;
    window.set_position(pos);
}