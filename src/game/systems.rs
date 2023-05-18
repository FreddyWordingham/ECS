use super::events::*;
use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

pub fn spawn_camera(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let windows = query.get_single().unwrap();
    let width = windows.width();
    let height = windows.height();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(width * 0.5, height * 0.5, 0.0),
        ..default()
    });
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
