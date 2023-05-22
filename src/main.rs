use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

mod level;

#[derive(Component)]
struct Position{
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Health(f32);

#[derive(Resource, Default)]
struct GameState {
    enemies: u8,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    hp: Health,
    pos: Position,
    _p: Player,
}

#[derive(Bundle)]
struct EnemyBundle {
    name: Name,
    hp: Health,
    pos: Position,
    _e: Enemy,
}


fn startup_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(PlayerBundle{
        name: Name("default_player".to_string()),
        hp: Health(100.0),
        pos: Position{x: 0.0, y: 0.0},
        _p: Player,
    });
    commands.spawn(EnemyBundle{
        name: Name("default_player".to_string()),
        hp: Health(100.0),
        pos: Position{x: 0.0, y: 0.0},
        _e: Enemy,
    });
    game_state.enemies = 2;
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system)
        .run();
}

