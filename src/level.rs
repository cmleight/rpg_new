use bevy::{pbr::PbrBundle, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct MainCamera;

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
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

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
    let player_spawn = PlayerSpawn {
        pos: Transform::from_xyz(0.0, 0.25, 0.0),
    };
    let enemy_spawns = vec![EnemySpawn {
        pos: Transform::from_xyz(1.0, 0.25, 0.0),
    }];
    for (i, spawn) in enemy_spawns.iter().enumerate() {
        info!("Spawning enemy at: {:?}", spawn);
        commands
            .spawn((
                EnemyBundle {
                    name: Name(format!("default_enemy_{i}").to_string()),
                    hp: Health(100.0),
                    body: PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.0).into()),
                        transform: spawn.pos,
                        ..default()
                    },
                    _e: Enemy,
                },
                RigidBody::Dynamic,
            ))
            .insert(Collider::cuboid(0.25, 0.25, 0.25))
            .insert(Damping {
                linear_damping: 0.5,
                angular_damping: 1.0,
            })
            .insert(GravityScale(0.0))
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled());
    }
    commands
        .spawn((
            PlayerBundle {
                name: Name("default_player".to_string()),
                hp: Health(100.0),
                body: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: player_spawn.pos,
                    ..default()
                },
                _p: Player,
            },
            RigidBody::Dynamic,
        ))
        .insert(Collider::cuboid(0.25, 0.25, 0.25))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(GravityScale(0.0))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
    // plane
    commands
        .spawn(LevelBundle {
            terrain: PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(5.0).into()),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..default()
            },
            player_spawn: player_spawn,
            enemy_spawn: EnemySpawnSet {
                spawns: enemy_spawns,
            },
        })
        .insert(RigidBody::Fixed);
}
