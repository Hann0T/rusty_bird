use bevy::prelude::*;

use crate::{physics::AutoMoving, collision::Collisionable};

const PIPE_HEIGHT: f32 = 150.0;
const PIPE_WIDTH: f32 = 75.0;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pipes);
    }
}

fn spawn_pipes(mut commands: Commands, assert_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let window_height = window.height();
    let _window_width = window.width();

    commands
        .spawn_bundle(SpriteBundle {
            texture: assert_server.load("pipe.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PIPE_WIDTH, PIPE_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, (-window_height / 2.0) + (PIPE_HEIGHT / 2.0), 0.0),
            ..default()
        })
        .insert(AutoMoving)
        .insert(Collisionable);
}
