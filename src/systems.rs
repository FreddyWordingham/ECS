use crate::{components::*, events::*, resources::*};
use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use rand::prelude::*;
use std::f32::consts::PI;

const PLAYER_SIZE: f32 = 64.0;
const PLAYER_SPEED: f32 = 500.0;

const INITIAL_NUMBER_OF_ENEMIES: usize = 10;
const ENEMY_SIZE: f32 = 64.0;
const ENEMY_SPEED: f32 = 350.0;
const MAX_NUMBER_OF_ENEMIES: usize = 20;

const INITIAL_NUMBER_OF_STARS: usize = 10;
const STAR_SIZE: f32 = 30.0;

pub fn spawn_camera(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let windows = query.get_single().unwrap();
    let width = windows.width();
    let height = windows.height();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(width * 0.5, height * 0.5, 0.0),
        ..default()
    });
}

pub fn spawn_player(
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

pub fn player_movement(
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

pub fn confine_player_movement(
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

pub fn player_enemy_collision(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let collision_distance = (PLAYER_SIZE + ENEMY_SIZE) * 0.5;

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

                game_over_event_writer.send(GameOver(score.0));
            }
        }
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    for _ in 0..INITIAL_NUMBER_OF_STARS {
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

pub fn player_hit_star(
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

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.0);
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.0.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.0.finished() {
        let window = window_query.get_single().unwrap();
        let width = window.width();
        let height = window.height();

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

pub fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_writer: EventReader<GameOver>) {
    for game_over in game_over_event_writer.iter() {
        println!("Game Over! Score: {}", game_over.0);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.0.push((String::from("Player"), event.0));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High scores:");
        for (i, (name, score)) in high_scores.0.iter().enumerate() {
            println!("{}. {}\t{}", i + 1, name, score);
        }
    }
}
