use bevy::prelude::*;

#[derive(Component)]
pub struct AffectedByGravity;

#[derive(Component)]
pub struct Velocity(pub f32);

struct Gravity(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(velocity_system)
            .add_system(gravity_system)
            .insert_resource(Gravity(350.0));
    }
}

fn velocity_system(
    gravity: Res<Gravity>,
    time: Res<Time>,
    mut query: Query<&mut Velocity, With<AffectedByGravity>>,
) {
    for mut velocity in query.iter_mut() {
        velocity.0 -= gravity.0 * time.delta_seconds();
    }
}

fn gravity_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<AffectedByGravity>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.y += velocity.0 * time.delta_seconds();
    }
}
