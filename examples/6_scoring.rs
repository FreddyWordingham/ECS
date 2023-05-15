use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;
use std::f32::consts::PI;

// == Settings ==
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;
const NUMBER_OF_ENEMIES: usize = 10;
const ENEMY_SPEED: f32 = 350.0;
const ENEMY_SIZE: f32 = 64.0;
const NUMBER_OF_STARS: usize = 100;
const STAR_SIZE: f32 = 30.0;

// == Main ==
fn main() {
    App::new()
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

// == Plugins ==
struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_enemies)
            .add_startup_system(spawn_stars)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(confine_enemy_movement)
            .add_system(player_enemy_collision)
            .add_system(player_hit_star)
            .add_system(update_score);
    }
}

// == Resources ==
#[derive(Resource)]
struct Score(u32);

impl Default for Score {
    fn default() -> Self {
        Self(0)
    }
}

// == Components ==
#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Enemy {
    direction: Vec2,
}

#[derive(Component)]
struct Star {}

// == Systems ==
fn spawn_camera(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let windows = query.get_single().unwrap();
    let width = windows.width();
    let height = windows.height();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(width * 0.5, height * 0.5, 0.0),
        ..default()
    });
}

fn spawn_player(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    commands.spawn((
        Player {},
        SpriteBundle {
            transform: Transform::from_xyz(width / 2.0, height / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
    ));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let width = window.width();
        let height = window.height();

        let half_player_size = PLAYER_SIZE * 0.5;
        let x_min = half_player_size;
        let x_max = width - half_player_size;
        let y_min = half_player_size;
        let y_max = height - half_player_size;

        if transform.translation.x < x_min {
            transform.translation.x = x_min;
        } else if transform.translation.x > x_max {
            transform.translation.x = x_max;
        }
        if transform.translation.y < y_min {
            transform.translation.y = y_min;
        } else if transform.translation.y > y_max {
            transform.translation.y = y_max;
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    for _ in 0..NUMBER_OF_ENEMIES {
        let mut theta = random::<f32>() * PI * 2.0;

        commands.spawn((
            Enemy {
                direction: Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5).normalize(),
            },
            SpriteBundle {
                transform: Transform::from_xyz(
                    random::<f32>() * width,
                    random::<f32>() * height,
                    0.0,
                ),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
        ));
    }
}

fn enemy_movement(mut query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn update_enemy_direction(
    mut query: Query<(&mut Enemy, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    let half_enemy_size = ENEMY_SIZE * 0.5;
    let x_min = half_enemy_size;
    let x_max = width - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = height - half_enemy_size;

    let sound_effect_0 = asset_server.load("audio/pluck_000.ogg");
    let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");

    for (mut enemy, transform) in query.iter_mut() {
        let mut changed_direction = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            changed_direction = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            changed_direction = true;
        }

        if changed_direction {
            let effect = match random::<bool>() {
                true => sound_effect_0.clone(),
                false => sound_effect_1.clone(),
            };
            audio.play(effect);
        }
    }
}

fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    let half_enemy_size = PLAYER_SIZE * 0.5;
    let x_min = half_enemy_size;
    let x_max = width - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = height - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        if transform.translation.x < x_min {
            transform.translation.x = x_min;
        } else if transform.translation.x > x_max {
            transform.translation.x = x_max;
        }
        if transform.translation.y < y_min {
            transform.translation.y = y_min;
        } else if transform.translation.y > y_max {
            transform.translation.y = y_max;
        }
    }
}

fn player_enemy_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let collision_distance = PLAYER_SIZE * 0.5 + ENEMY_SIZE * 0.5;

    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            if player_transform
                .translation
                .distance(enemy_transform.translation)
                < collision_distance
            {
                commands.entity(player_entity).despawn();
                let effect = asset_server.load("audio/explosion_crunch_000.ogg");
                audio.play(effect);
            }
        }
    }
}

fn spawn_stars(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    for _ in 0..NUMBER_OF_STARS {
        commands.spawn((
            Star {},
            SpriteBundle {
                transform: Transform::from_xyz(
                    random::<f32>() * width,
                    random::<f32>() * height,
                    -1.0,
                ),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
        ));
    }
}

fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let collision_distance = PLAYER_SIZE * 0.5 + STAR_SIZE * 0.5;

        for (star, star_transform) in star_query.iter() {
            if player_transform
                .translation
                .distance(star_transform.translation)
                < collision_distance
            {
                score.0 += 1;

                commands.entity(star).despawn();

                let effect = asset_server.load("audio/laser_large_000.ogg");
                audio.play(effect);
            }
        }
    }
}

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.0);
    }
}
