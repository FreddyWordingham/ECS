use super::resources::*;
use crate::game::events::*;
use bevy::prelude::*;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.0);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.0.push((String::from("Player"), event.0));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High scores:");
        for (i, (name, score)) in high_scores.0.iter().enumerate() {
            println!("{}. {}\t{}", i + 1, name, score);
        }
    }
}
