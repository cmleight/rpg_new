use bevy::prelude::*;

mod level;
mod common;


fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    // info!("inputs: {:?}", keyboard_input);

    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}

fn startup_system(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle{
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
        .add_startup_system(level::setup_level)
        .add_startup_system(startup_system)
        .add_system(keyboard_input_system)
        .run();
}

