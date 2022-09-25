use bevy::prelude::*;
use bevy::sprite::{Sprite, SpriteBundle};

use crate::physics::{AffectedByGravity, Velocity};

#[derive(Component)]
struct CanFly;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_bird)
            .add_system(bird_controller);
        // .add_system(bird_collision);
    }
}

fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let window_width = window.width();

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("bird.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(55.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(-(window_width / 2.0) + 100.0, 0.0, 0.0),
            ..default()
        })
        .insert(CanFly)
        .insert(Velocity(0.0))
        .insert(AffectedByGravity);
}

fn bird_controller(
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<CanFly>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        if key_input.just_pressed(KeyCode::Space) {
            transform.rotation = Quat::from_rotation_z(0.4);
            velocity.0 = 180.0;
        }

        if key_input.just_released(KeyCode::Space) {
            transform.rotation = Quat::from_rotation_z(-0.4);
        }
    }
}

// fn bird_collision() {
//     todo!()
// }
