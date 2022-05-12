use crate::util::constant::*;
use crate::model::notequeue::*;

use std::collections::VecDeque;
use std::cmp::max;
use std::io::{BufRead, BufReader};
use ggez::{Context, filesystem, GameResult, timer};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::event::KeyCode;
use ggez::mint::Point2;
use crate::SongState;

pub fn get_notes(ctx: &mut Context) -> Vec<NoteQueue> {
    let tja = load_tja(ctx, TJA_PATH);
    let mut red_queue = VecDeque::new();
    let mut blue_queue = VecDeque::new();
    let bpm = 180.0;
    let bar = 240.0 * SPEED / bpm;
    let start = GOAL + 2.525 * SPEED;
    (0..tja.len()).for_each( |i| {
        let measure = &tja[i];
        let beat = bar / (measure.len() as f32);
        (0..measure.len()).for_each( |j| {
            let note = measure.as_bytes()[j] as char;
            if note == '1' || note == '3' || note == '5' {
                red_queue.push_back(start + (i as f32) * bar + (j as f32) * beat)
            }
            if note == '2' || note == '4' {
                blue_queue.push_back(start + (i as f32) * bar + (j as f32) * beat)
            }
        });
    });
    vec![
        NoteQueue {
            queue: red_queue,
            color: Color::RED,
        },
        NoteQueue {
            queue: blue_queue,
            color: Color::CYAN,
        }
    ]
}

pub fn load_tja(ctx: &mut Context, path: &str) -> Vec<String> {
    let mut notes = Vec::new();
    let tja = filesystem::open(ctx, path).unwrap();
    let lines = BufReader::new(tja).lines();
    for line_result in lines {
        if let Ok(mut line) = line_result {
            if line == "#END" { break }
            if line.is_empty() { continue }
            if line.ends_with(',') {
                line.pop();
                notes.push(line);
            }
        }
    }
    notes
}

pub fn move_notes(state: &mut SongState, ctx: &mut Context) {
    let dt = timer::delta(ctx).as_secs_f32();
    state.queues.iter_mut()
        .for_each(|q| q.queue.iter_mut()
            .for_each(|x| *x -= SPEED * dt));

    let oq = state.queues.iter_mut()
        .find(|q| q.queue.iter()
            .any(|x| *x < GOAL - BAD_WINDOW));
    if oq.is_some() {
        oq.unwrap().queue.pop_front();
        state.bad += 1;
        state.combo = 0;
    }
}

pub fn hit_notes(state: &mut SongState, keycode: KeyCode) {
    let red_down = keycode == KeyCode::F || keycode == KeyCode::J;
    let blue_down = keycode == KeyCode::D || keycode == KeyCode::K;
    if !red_down && !blue_down { return };
    let queue = if red_down { &mut state.queues[0].queue } else { &mut state.queues[1].queue };
    if within_window(queue, GOOD_WINDOW) {
        state.timings.push(
            queue.pop_front().unwrap());
        state.good += 1;
        state.combo += 1;
    } else if within_window(queue, OKAY_WINDOW) {
        state.timings.push(
            queue.pop_front().unwrap());
        state.okay += 1;
        state.combo += 1;
    } else if within_window(queue, BAD_WINDOW) {
        state.timings.push(
            queue.pop_front().unwrap());
        state.bad += 1;
        state.combo = 0;
    }
    state.max_combo = max(state.max_combo, state.combo);
}

pub fn song_is_over(state: &SongState) -> bool {
    state.queues.iter().all(|q| q.queue.is_empty())
}

pub fn end_song(state: &mut SongState) {
    let mut avg = 0.0;
    let mut max = -1000.0;
    let mut min = 1000.0;
    for timing in state.timings.iter() {
        let millis = (*timing - GOAL) * 1000.0 / SPEED;
        if millis < min { min = millis }
        if millis > max { max = millis }
        avg += millis;
    }
    avg = avg / (state.timings.len() as f32);
    println!("\n    Min: {} ms\n    Max: {} ms\nAverage: {} ms", min, max, avg);
}

pub fn within_window(queue: &VecDeque<f32>, window: f32) -> bool {
    queue.front()
        .filter(|x| **x < GOAL + window)
        .filter(|x| **x > GOAL - window)
        .is_some()
}

pub fn draw_goal(ctx: &mut Context) -> GameResult {
    let goal = graphics::Mesh::new_circle(
        ctx, DrawMode::stroke(GOAL_THICKNESS),
        Point2::from([GOAL, LANE]),
        RADIUS, 0.1, Color::WHITE)?;
    graphics::draw(ctx, &goal, DrawParam::default())
}

pub fn draw_notes(ctx: &mut Context, queue: &NoteQueue) -> GameResult {
    let (width, _) = graphics::drawable_size(ctx);
    queue.queue.iter()
        .filter(|x| **x < width + RADIUS)
        .map(|x| draw_note(ctx, x, queue.color))
        .fold(Ok(()), GameResult::and)
}

pub fn draw_note(ctx: &mut Context, x: &f32, color: Color) -> GameResult {
    let circle = graphics::Mesh::new_circle(
        ctx, DrawMode::stroke(NOTE_THICKNESS),
        Point2::from([*x, LANE]),
        RADIUS, 1.0, color)?;
    graphics::draw(ctx, &circle, DrawParam::default())
}

pub fn draw_score(ctx: &mut Context, good: u32, okay: u32, bad: u32, combo: u32, max_combo: u32) -> GameResult {
    let score_text = graphics::Text::new(
        format!("Good: {}\nOkay: {}\nBad: {}\n\nCombo: {}\nMax Combo: {}",
                 good,     okay,     bad,       combo,     max_combo));
    graphics::draw(ctx, &score_text, DrawParam::default())
}