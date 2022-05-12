use crate::util::constant::*;
use crate::util::song::*;
use crate::model::notequeue::*;

use std::path::Path;
use ggez::{audio, Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{EventHandler, KeyCode, KeyMods};

pub struct SongState {
    pub queues:  Vec<NoteQueue>,
    pub good: u32,
    pub okay: u32,
    pub bad: u32,
    pub combo: u32,
    pub max_combo: u32,
    pub timings: Vec<f32>,
    pub song: audio::Source,
}

impl SongState {
    pub fn new(ctx: &mut Context) -> GameResult<SongState> {
        let queues = get_notes(ctx);
        Ok(SongState {
            queues,
            good: 0,
            okay: 0,
            bad: 0,
            combo: 0,
            max_combo: 0,
            timings: Vec::new(),
            song: audio::Source::new(ctx, Path::new(SONG_PATH))?,
        })
    }
}

impl EventHandler for SongState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        move_notes(self, ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        draw_goal(ctx)?;
        draw_score(ctx, self.good, self.okay, self.bad, self.combo, self.max_combo)?;
        self.queues.iter()
            .map(|q| draw_notes(ctx, q))
            .fold(Ok(()), GameResult::and)?;
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        if repeat { return };
        hit_notes(self, keycode);
        if song_is_over(self) { end_song(self) };
    }
}