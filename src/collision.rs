use bevy::prelude::*;

use crate::bird::Bird;

#[derive(Component)]
pub struct Collisionable;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_system);
    }
}

fn collision_system(
    //windows: Res<Windows>,
    player: Query<(&Transform, &Sprite), With<Bird>>,
    collisionable_sprite: Query<(&Transform, &Sprite), With<Collisionable>>,
) {
    let (player_transform, player_sprite) = player.single();
    for (transform, sprite) in collisionable_sprite.iter() {
        let player_collision = (player_transform.translation.x
            + player_sprite.custom_size.unwrap().x / 2.0)
            > (transform.translation.x - sprite.custom_size.unwrap().x / 2.0)
            && (player_transform.translation.x - player_sprite.custom_size.unwrap().x / 2.0)
                < (transform.translation.x + sprite.custom_size.unwrap().x / 2.0)
            && (player_transform.translation.y + player_sprite.custom_size.unwrap().y / 2.0)
                > (transform.translation.y - sprite.custom_size.unwrap().y / 2.0)
            && (player_transform.translation.y - player_sprite.custom_size.unwrap().y / 2.0)
                < (transform.translation.y + sprite.custom_size.unwrap().y / 2.0);

        println!("player Collision {}", player_collision);
    }
}
