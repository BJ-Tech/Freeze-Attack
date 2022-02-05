// demon attack with freeze functions in rust using ggez

// imports
extern crate ggez;
use ggez::graphics::{self, Color, DrawMode, Rect, Text, TextFragment};
use ggez::{conf, event, Context, GameResult};

struct MainState {}

impl MainState {
    fn new() -> Self {
        MainState {}
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // make background black
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));
        // draw player at the bottom of the screen
        let (x, y) = graphics::drawable_size(ctx);
        let player_rect = graphics::Rect::new(x / 2.0 - 50.0, y - 50.0, 100.0, 100.0);
        let player_rect_mesh = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            player_rect,
            Color::from_rgb(255, 255, 255),
        )?;
        graphics::draw(
            ctx,
            &player_rect_mesh,
            (ggez::mint::Point2 { x: 0.0, y: 0.0 },),
        )?;
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
