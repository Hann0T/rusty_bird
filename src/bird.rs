use bevy::prelude::*;
use bevy::sprite::{Sprite, SpriteBundle};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_bird)
            .add_system(bird_controller);
    }
}

#[derive(Component)]
struct CanFly;

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
        .insert(CanFly);
}

fn bird_controller(keyinput: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<CanFly>>) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= 5.0;
        transform.rotation = Quat::from_rotation_z(-0.4);

        if keyinput.pressed(KeyCode::Space) {
            transform.rotation = Quat::from_rotation_z(0.4);
            transform.translation.y += 15.0;
        }
    }
}
