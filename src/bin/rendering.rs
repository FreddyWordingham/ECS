use bevy::prelude::*;

// == Main ==
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

// == Plugins ==
struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {}
}

// == Components ==

// == Systems ==
