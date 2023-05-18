use bevy::prelude::*;

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
