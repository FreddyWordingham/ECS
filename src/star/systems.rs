use super::{components::Star, resources::*, INITIAL_NUMBER_OF_STARS};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

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
