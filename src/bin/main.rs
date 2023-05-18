use bevy::prelude::*;

use ecs::enemy::EnemyPlugin;
use ecs::game::GamePlugin;
use ecs::player::PlayerPlugin;
use ecs::score::ScorePlugin;
use ecs::star::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .run();
}
