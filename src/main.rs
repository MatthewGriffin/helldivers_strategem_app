mod process_input;
mod process_touch_event;
mod setup_interface;
mod touch_detection;
mod touch_resource;

use bevy::{
    app::{App, Startup, Update},
    prelude::{default, PluginGroup},
    window::{Window, WindowMode, WindowPlugin},
    winit::WinitSettings,
    DefaultPlugins,
};
use process_touch_event::ProcessTouchEvent;
use setup_interface::setup_interface;
use touch_resource::TouchResource;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(
            Update,
            (
                touch_detection::touch_detection,
                process_input::process_input,
            ),
        )
        .add_systems(Startup, setup_interface)
        .insert_resource(TouchResource::default())
        .add_event::<ProcessTouchEvent>()
        .run();
}
