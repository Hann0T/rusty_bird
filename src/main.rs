mod bird;
mod camera;
mod physics;
mod pipes;
mod collision;

use bevy::prelude::*;
use bird::BirdPlugin;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use physics::PhysicsPlugin;
use pipes::PipesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(BirdPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PipesPlugin)
        .add_plugin(CollisionPlugin)
        .run();
}
