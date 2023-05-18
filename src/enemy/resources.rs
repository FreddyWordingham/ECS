use super::ENEMY_SPAWN_INTERVAL;
use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            ENEMY_SPAWN_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}
