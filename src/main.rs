mod touch_detection;

use bevy::{
    app::{App, Update},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, touch_detection::touch_detection)
        .run();
}
