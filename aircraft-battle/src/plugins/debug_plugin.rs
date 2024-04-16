use bevy::prelude::*;

use crate::state::{GameState,MenuState};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_game_state);
    }
}

fn print_game_state(game_state: ResMut<State<GameState>>,app_state: ResMut<State<MenuState>>, keyboard_input: Res<ButtonInput<KeyCode>>,) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        println!("GameState: {}",game_state.get());
        match app_state.get() {
            MenuState::MainMenu => println!("main"),
            MenuState::Playing => println!("playing"),
        }
    }
}
