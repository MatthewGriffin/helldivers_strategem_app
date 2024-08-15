mod touch_detection;
mod touch_resource;

use bevy::{
    app::{App, Update},
    DefaultPlugins,
};
use touch_resource::TouchResource;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, touch_detection::touch_detection)
        .insert_resource(TouchResource::default())
        .run();
}
