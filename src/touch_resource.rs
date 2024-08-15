use bevy::{math::Vec2, prelude::Resource};

#[derive(Resource)]
pub struct TouchResource {
    pub touch_start: Vec2,
    pub touch_end: Vec2,
}

// custom implementation for unusual values
impl Default for TouchResource {
    fn default() -> Self {
        TouchResource {
            touch_start: Vec2::new(0.0, 0.0),
            touch_end: Vec2::new(0.0, 0.0),
        }
    }
}
