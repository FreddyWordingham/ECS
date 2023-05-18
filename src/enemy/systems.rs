use super::{
    components::*, resources::*, ENEMY_SIZE, ENEMY_SPEED, INITIAL_NUMBER_OF_ENEMIES,
    MAX_NUMBER_OF_ENEMIES,
};
use crate::player::PLAYER_SIZE;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;
use std::f32::consts::PI;

pub fn spawn_enemies(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    for _ in 0..INITIAL_NUMBER_OF_ENEMIES {
        let theta = random::<f32>() * PI * 2.0;
        let direction = Vec2::new(theta.cos(), theta.sin());
        let offset = direction * ((ENEMY_SIZE + PLAYER_SIZE) * 0.5001);

        commands.spawn((
            Enemy { direction },
            SpriteBundle {
                transform: Transform::from_xyz(
                    (width * 0.5) + offset.x,
                    (height * 0.5) + offset.y,
                    0.0,
                ),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
        ));
    }
}

pub fn enemy_movement(mut query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
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

    let mut changed_direction = false;
    for (mut enemy, transform) in query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            changed_direction = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            changed_direction = true;
        }
    }

    if changed_direction {
        let effect = match random::<bool>() {
            true => sound_effect_0.clone(),
            false => sound_effect_1.clone(),
        };
        audio.play(effect);
    }
}

pub fn confine_enemy_movement(
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

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.0.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    enemy_query: Query<&Enemy>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.0.finished() {
        if enemy_query.iter().count() >= MAX_NUMBER_OF_ENEMIES {
            return;
        }

        let window = window_query.get_single().unwrap();
        let width = window.width();
        let height = window.height();

        let theta = random::<f32>() * 2.0 * PI;
        commands.spawn((
            Enemy {
                direction: Vec2::new(theta.cos(), theta.sin()),
            },
            SpriteBundle {
                transform: Transform::from_xyz(
                    random::<f32>() * width,
                    random::<f32>() * height,
                    -1.0,
                ),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
        ));
    }
}
