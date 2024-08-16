mod process_input;
mod process_touch_event;
mod touch_detection;
mod touch_resource;

use bevy::{
    app::{App, Update},
    DefaultPlugins,
};
use process_touch_event::ProcessTouchEvent;
use touch_resource::TouchResource;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            (
                touch_detection::touch_detection,
                process_input::process_input,
            ),
        )
        .insert_resource(TouchResource::default())
        .add_event::<ProcessTouchEvent>()
        .run();
}
