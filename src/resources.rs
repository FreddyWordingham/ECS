use bevy::prelude::*;

const STAR_SPAWN_TIME: f32 = 0.25;
const ENEMY_SPAWN_TIME: f32 = 2.5;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer(pub Timer);
impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct Score(pub u32);
impl Default for Score {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Resource)]
pub struct HighScores(pub Vec<(String, u32)>);
impl Default for HighScores {
    fn default() -> Self {
        Self(vec![("Bens".to_string(), 2), ("Philber".to_string(), 5)])
    }
}
