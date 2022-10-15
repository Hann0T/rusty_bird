use bevy::prelude::*;

use crate::{gamedata::GameData, gamestate::GameState};

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct PlayerStateText;

pub struct UiTextPlugin;

impl Plugin for UiTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_text)
            .add_system(update_score_text)
            .add_system(update_player_state_text);
    }
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "SCORE: ",
                    TextStyle {
                        font: asset_server.load("fonts/UbuntuMono-R.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/UbuntuMono-R.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }),
            ])
            .with_text_alignment(TextAlignment::BOTTOM_LEFT),
        )
        .insert(ScoreText);

    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "STATE: ",
                    TextStyle {
                        font: asset_server.load("fonts/UbuntuMono-R.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/UbuntuMono-R.ttf"),
                    font_size: 40.0,
                    color: Color::GOLD,
                }),
            ])
            .with_text_alignment(TextAlignment::BOTTOM_RIGHT),
        )
        .insert(PlayerStateText);
}

fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>, game_data: Res<GameData>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", game_data.score);
    }
}

fn update_player_state_text(
    mut query: Query<&mut Text, With<PlayerStateText>>,
    game_data: Res<GameData>,
) {
    for mut text in query.iter_mut() {
        match game_data.game_state {
            GameState::Playing => {
                text.sections[1].value = String::from("Playing");
            }
            GameState::Menu => {
                text.sections[1].value = String::from("Menu");
            }
            GameState::Dead => {
                text.sections[1].value = String::from("Dead");
            }
        }
    }
}
