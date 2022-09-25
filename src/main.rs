mod bird;
mod camera;
mod physics;

use bevy::prelude::*;
use bird::BirdPlugin;
use camera::CameraPlugin;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(BirdPlugin)
        .add_plugin(PhysicsPlugin)
        .run();
}
