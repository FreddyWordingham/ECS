use super::{components::*, PLAYER_SIZE, PLAYER_SPEED};
use crate::{
    enemy::{components::*, ENEMY_SIZE},
    game::events::*,
    score::resources::*,
    star::{components::*, STAR_SIZE},
};
use bevy::{prelude::*, window::PrimaryWindow};

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

pub fn player_hit_enemy(
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
