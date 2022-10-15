mod bird;
mod camera;
mod collision;
mod gamedata;
mod gamestate;
mod physics;
mod pipes;
pub mod uitext;

use bevy::prelude::*;
use bird::BirdPlugin;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use gamedata::GameData;
use gamestate::{GameState, GameStatePlugin};
use physics::PhysicsPlugin;
use pipes::PipesPlugin;
use uitext::UiTextPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(UiTextPlugin)
        .add_plugin(BirdPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PipesPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GameStatePlugin)
        .insert_resource(GameData {
            game_state: GameState::Menu,
            score: 0,
        })
        .run();
}
