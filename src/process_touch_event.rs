use bevy::{math::Vec2, prelude::Event};

#[derive(Event)]
pub struct ProcessTouchEvent {
    pub start_end_diff: Vec2,
}
