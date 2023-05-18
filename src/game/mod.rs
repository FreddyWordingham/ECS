use bevy::prelude::*;

pub mod events;
mod systems;

use events::*;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_startup_system(spawn_camera)
            .add_system(exit_game)
            .add_system(handle_game_over);
    }
}
