use bevy::prelude::*;
use bevy::sprite::{Sprite, SpriteBundle};

use crate::gamedata::GameData;
use crate::gamestate::GameState;
use crate::physics::{AffectedByGravity, Velocity};

#[derive(Component)]
struct CanFly;

#[derive(Component)]
pub struct Bird;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_bird)
            .add_system(bird_controller)
            .add_system(bird_auto_jump)
            .add_system(bird_bounds);
    }
}

fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("bird.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(55.0, 50.0)),
                ..default()
            },
            transform: restart_bird_position(window),
            ..default()
        })
        .insert(CanFly)
        .insert(Bird)
        .insert(Velocity(Vec2::new(0.0, 0.0)))
        .insert(AffectedByGravity);
}

pub fn restart_bird_position(window: &Window) -> Transform {
    let window_width = window.width();
    Transform::from_xyz(-(window_width / 2.0) + 100.0, 0.0, 0.0)
}

fn bird_controller(
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<CanFly>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        if key_input.just_pressed(KeyCode::Space) {
            transform.rotation = Quat::from_rotation_z(0.4);
            velocity.0.y = 180.0;
        }

        if velocity.0.y > 0.0 {
            transform.rotation = Quat::from_rotation_z(0.4);
        } else {
            transform.rotation = Quat::from_rotation_z(-0.4);
        }
    }
}

fn bird_auto_jump(
    game_data: Res<GameData>,
    mut query: Query<(&mut Transform, &mut Velocity), With<CanFly>>,
) {
    if let GameState::Menu = game_data.game_state {
        for (mut transform, mut velocity) in query.iter_mut() {
            if transform.translation.y < 0.0 {
                transform.rotation = Quat::from_rotation_z(0.4);
                velocity.0.y = 180.0;
            }
        }
    }
}

fn bird_bounds(
    windows: Res<Windows>,
    mut game_data: ResMut<GameData>,
    query: Query<(&Transform, &Sprite), With<CanFly>>,
) {
    let window = windows.primary();
    let height = window.height();
    let half_window_height = height / 2.0;

    for (transform, sprite) in query.iter() {
        let sprite_y_size = sprite.custom_size.unwrap().y;

        if (transform.translation.y - (sprite_y_size / 2.0)) >= half_window_height
            || (transform.translation.y - (sprite_y_size / 2.0)) <= -(half_window_height)
        {
            game_data.game_state = GameState::Dead;
        }
    }
}
