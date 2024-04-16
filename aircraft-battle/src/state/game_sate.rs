
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    SatrtGame,
    InGame,
    Paused,
    GameOver,
}


impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GameState::SatrtGame => f.write_str("StartGame"),
            GameState::InGame => f.write_str("InGame"),
            GameState::Paused => f.write_str("Paused"),
            GameState::GameOver => f.write_str("GameOver"),
        }
    }
}