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

struct MainState {
}

impl MainState {
    fn new() -> Self {
        MainState {
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}


fn main() -> GameResult {
    // context builder
    let cb = ggez::ContextBuilder::new("Freeze-Attack", "Freeze-Attack");
    let (ctx, event_loop) = cb.build()?; // build context and event loop


    //graphics::set_window_title(ctx, "Freeze-Attack");

    // create game state
    let mut state = MainState::new();
    
    // run game
    event::run(ctx, event_loop, state);

    Ok(())

}