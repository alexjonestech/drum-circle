use std::collections::VecDeque;
use ggez::graphics::Color;

pub struct NoteQueue {
    pub queue: VecDeque<f32>,
    pub color: Color,
}