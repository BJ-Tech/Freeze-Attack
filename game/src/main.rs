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

    fn rectangle ( ctx: &mut Context,) -> GameResult {
        // draws a rectangle to the bottom center of the screen
        let (x, y) = graphics::drawable_size(ctx);
        let rect = graphics::Rect::new(x / 2.0 - 50.0, y - 50.0, 100.0, 100.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
        graphics::present(ctx)?;

        
        Ok(())
    }
}


fn main() -> GameResult {
    // context builder
    let cb = ggez::ContextBuilder::new("Freeze-Attack", "Freeze-Attack");
    let (ctx, event_loop) = cb.build()?; // build context and event loop


    //gives winddow a title
    graphics::set_window_title(&ctx, "Freeze-Attack");

    // create game state
    let state = MainState::new();
    
    // run game
    event::run(ctx, event_loop, state);

}
