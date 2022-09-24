mod bird;
mod camera;

use bevy::prelude::*;
use bird::BirdPlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(BirdPlugin)
        .run();
}
