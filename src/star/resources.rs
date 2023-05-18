use super::STAR_SPAWN_INTERVAL;
use bevy::prelude::*;

#[derive(Resource)]
pub struct StarSpawnTimer(pub Timer);
impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            STAR_SPAWN_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}
