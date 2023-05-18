use bevy::prelude::*;

use ecs::{events::*, resources::*, systems::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<StarSpawnTimer>()
            .init_resource::<EnemySpawnTimer>()
            .init_resource::<HighScores>()
            .add_event::<GameOver>()
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_enemies)
            .add_startup_system(spawn_stars)
            .add_system(exit_game)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(confine_enemy_movement)
            .add_system(player_enemy_collision)
            .add_system(player_hit_star)
            .add_system(update_score)
            .add_system(tick_star_spawn_timer)
            .add_system(tick_enemy_spawn_timer)
            .add_system(spawn_stars_over_time)
            .add_system(spawn_enemies_over_time)
            .add_system(handle_game_over)
            .add_system(update_high_scores)
            .add_system(high_scores_updated);
    }
}
