use crate::level;
use bevy::prelude::*;

const MOVE_SPEED: f32 = 0.5;

#[derive(Resource)]
pub struct KeyMap {
    // config_handle: Handle<Font>,
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
}

pub fn setup_keymap(
    mut commands: Commands,
    // server: Res<AssetServer>,
) {
    // let handle: Handle<Font> = server.load("key_config");

    // we can store the handle in a resource:
    //  - to prevent the asset from being unloaded
    //  - if we want to use it to access the asset later
    commands.insert_resource(KeyMap {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,
    });
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    key_map: Res<KeyMap>,
    mut player_position: Query<(&level::Player, &mut Transform)>,
    timer: Res<Time>,
) {
    let (_, mut transform) = player_position.single_mut();
    let mut adjust = Vec3::ZERO;

    if keyboard_input.pressed(key_map.up) {
        adjust += transform.forward();
    }
    if keyboard_input.pressed(key_map.down) {
        adjust += transform.back();
    }
    if keyboard_input.pressed(key_map.left) {
        adjust += transform.left();
    }
    if keyboard_input.pressed(key_map.right) {
        adjust += transform.right();
    }
    let final_adjust = adjust.normalize_or_zero() * MOVE_SPEED * timer.delta_seconds();
    transform.translation += final_adjust;
}

pub fn my_cursor_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<level::MainCamera>>,
    mut player_position: Query<(&level::Player, &mut Transform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    let (_, mut player) = player_position.single_mut();
    let player_transform = player.translation;

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.get_point(ray.intersect_plane(player_transform, Vec3::Y).unwrap()))
    {
        player.look_at(world_position, Vec3::Y)
    }
}
