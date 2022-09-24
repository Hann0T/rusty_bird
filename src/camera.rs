use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
        // .add_system(camera_controller);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

// fn camera_controller(mut camera: Query<&mut Transform, With<Camera>>) {
//     for mut transform in camera.iter_mut() {
//         transform.translation.x += 1.0;
//     }
// }
