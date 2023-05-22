use bevy::{pbr::PbrBundle, prelude::*};

#[derive(Component, Debug)]
pub struct PlayerSpawn {
    pub pos: Transform,
}

#[derive(Component, Debug)]
pub struct EnemySpawn {
    pub pos: Transform,
}

#[derive(Component)]
pub struct EnemySpawnSet {
    pub spawns: Vec<EnemySpawn>,
}

#[derive(Bundle)]
pub struct LevelBundle {
    player_spawn: PlayerSpawn,
    enemy_spawn: EnemySpawnSet,
    terrain: PbrBundle,
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Health(f32);

#[derive(Resource, Default)]
struct GameState {}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    hp: Health,
    body: PbrBundle,
    _p: Player,
}

#[derive(Bundle)]
struct EnemyBundle {
    name: Name,
    hp: Health,
    body: PbrBundle,
    _e: Enemy,
}

pub fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_spawn = PlayerSpawn {
        pos: Transform::from_xyz(0.0, 0.25, 0.0),
    };
    let enemy_spawns = vec![EnemySpawn {
        pos: Transform::from_xyz(1.0, 0.5, 0.0),
    }];
    for (i, spawn) in enemy_spawns.iter().enumerate() {
        info!("Spawning enemy at: {:?}", spawn);
        commands.spawn(EnemyBundle {
            name: Name(format!("default_enemy_{i}").to_string()),
            hp: Health(100.0),
            body: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.0).into()),
                transform: spawn.pos,
                ..default()
            },
            _e: Enemy,
        });
    }
    commands.spawn(PlayerBundle {
        name: Name("default_player".to_string()),
        hp: Health(100.0),
        body: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: player_spawn.pos,
            ..default()
        },
        _p: Player,
    });
    // plane
    commands.spawn(LevelBundle {
        terrain: PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        player_spawn: player_spawn,
        enemy_spawn: EnemySpawnSet {
            spawns: enemy_spawns,
        },
    });
}
