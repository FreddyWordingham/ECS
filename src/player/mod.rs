use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(player_hit_enemy)
            .add_system(player_hit_star);
    }
}
