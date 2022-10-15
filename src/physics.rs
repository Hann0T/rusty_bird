use bevy::prelude::*;

use crate::{gamedata::GameData, gamestate::GameState};

#[derive(Component)]
pub struct AffectedByGravity;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct AutoMoving;

struct Gravity(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(velocity_vertical_system)
            .add_system(velocity_horizontal_system)
            .add_system(gravity_system)
            .insert_resource(Gravity(350.0));
    }
}

fn velocity_vertical_system(
    gravity: Res<Gravity>,
    time: Res<Time>,
    mut query: Query<&mut Velocity, With<AffectedByGravity>>,
) {
    for mut velocity in query.iter_mut() {
        velocity.0.y -= gravity.0 * time.delta_seconds();
    }
}

fn velocity_horizontal_system(
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<&mut Transform, With<AutoMoving>>,
) {
    if let GameState::Playing = game_data.game_state {
        for mut transform in query.iter_mut() {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}

fn gravity_system(
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(&Velocity, &mut Transform), With<AffectedByGravity>>,
) {
    match game_data.game_state {
        GameState::Menu => {
            for (velocity, transform) in query.iter_mut() {
                increment_gravity(transform, velocity, &time);
            }
        }
        GameState::Playing => {
            for (velocity, transform) in query.iter_mut() {
                increment_gravity(transform, velocity, &time);
            }
        }
        GameState::Dead => {}
    }
}

fn increment_gravity(mut transform: Mut<Transform>, velocity: &Velocity, time: &Time) {
    transform.translation.y += velocity.0.y * time.delta_seconds();
}
