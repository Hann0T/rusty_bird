use bevy::prelude::*;

use crate::{
    bird::{restart_bird_position, Bird},
    gamedata::GameData,
};

pub enum GameState {
    Menu,
    Playing,
    Dead,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_state_handler).add_system(restart_game);
    }
}

fn game_state_handler(mut game_data: ResMut<GameData>, key_input: Res<Input<KeyCode>>) {
    match game_data.game_state {
        GameState::Menu => {
            if key_input.just_pressed(KeyCode::Space) {
                game_data.game_state = GameState::Playing;
            }
        }
        GameState::Dead => {
            if key_input.just_pressed(KeyCode::Space) {
                game_data.game_state = GameState::Menu;
            }
        }
        GameState::Playing => {}
    }
}

fn restart_game(
    mut game_data: ResMut<GameData>,
    key_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<Bird>>,
) {
    if let GameState::Dead = game_data.game_state {
        if key_input.just_pressed(KeyCode::Space) {
            game_data.game_state = GameState::Menu;
            let window = windows.primary();
            for mut transform in query.iter_mut() {
                *transform = restart_bird_position(window);
            }
        }
    }
}
