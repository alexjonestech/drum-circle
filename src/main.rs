mod util;
mod state;
mod model;

use ggez::{ContextBuilder, event, GameResult};
use ggez::audio::SoundSource;
use ggez::conf::{WindowMode, WindowSetup};
use crate::state::song::SongState;


fn main() -> GameResult {
    let (mut ctx, event_loop) =
        ContextBuilder::new("Drum Circle", "Circle Guy")
            .window_setup(
                WindowSetup::default()
                    .title("Drum Circle")
                    .vsync(false))
            .window_mode(
                WindowMode::default()
                    .dimensions(3000.0, 600.0))
            .build()?;
    let mut state = SongState::new(&mut ctx)?;
    state.song.play(&ctx)?;
    event::run(ctx, event_loop, state);
}