// demon attack with freeze functions in rust using ggez

// imports
extern crate ggez;
use ggez::graphics::{self, Color, DrawMode, Rect, Text, TextFragment};
use ggez::input::keyboard::{self, KeyCode};
use ggez::{conf, event, Context, GameResult};
//use ggez::nalgreba as na;

struct MainState {
    // position player bottom of the screen
    player_x: f32,
    player_y: f32,
}

impl MainState {
    fn new() -> Self {
        MainState {
            player_x: 0.0,
            player_y: 0.0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // player movement
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.player_x -= 10.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.player_x += 10.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // make background black
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        // draw player at the bottom of the screen with player movement
        let player_rect = Rect::new(self.player_x, self.player_y, 50.0, 50.0);
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
