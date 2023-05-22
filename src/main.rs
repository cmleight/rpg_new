use bevy::prelude::*;

mod config;
mod inputs;
mod level;

fn startup_system(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(inputs::setup_keymap)
        .add_startup_system(level::setup_level)
        .add_startup_system(startup_system)
        .add_system(inputs::keyboard_input_system)
        .run();
}
