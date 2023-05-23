use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod config;
mod inputs;
mod level;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(inputs::setup_keymap)
        .add_startup_system(level::setup_level)
        .add_system(inputs::keyboard_input_system)
        .add_system(inputs::my_cursor_system)
        .run();
}
