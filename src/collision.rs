use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

use crate::{bird::Bird, gamedata::GameData, gamestate::GameState};

#[derive(Component)]
pub struct Collisionable;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_collision_system)
            .add_system(score_collision_system);
    }
}

fn player_collision_system(
    mut game_data: ResMut<GameData>,
    player: Query<(&Transform, &Sprite), With<Bird>>,
    collisionable_sprite: Query<(&Transform, &Sprite), With<Collisionable>>,
) {
    let (player_transform, player_sprite) = player.single();
    for (transform, sprite) in collisionable_sprite.iter() {
        let player_collision = collide(
            player_transform.translation,
            player_sprite.custom_size.unwrap(),
            transform.translation,
            sprite.custom_size.unwrap(),
        );

        if player_collision.is_some() {
            game_data.game_state = GameState::Dead;
        }
    }
}

fn score_collision_system(
    mut game_data: ResMut<GameData>,
    player: Query<(&Transform, &Sprite), With<Bird>>,
    collisionable_sprite: Query<(&Transform, &Sprite), With<Collisionable>>,
) {
    if let GameState::Playing = game_data.game_state {
        let (player_transform, player_sprite) = player.single();
        for (transform, sprite) in collisionable_sprite.iter() {
            let player_collision = (player_transform.translation.x
                + player_sprite.custom_size.unwrap().x)
                > (transform.translation.x + sprite.custom_size.unwrap().x / 2.0)
                && (player_transform.translation.x + player_sprite.custom_size.unwrap().x)
                    < (transform.translation.x + sprite.custom_size.unwrap().x / 2.0 + 2.0);

            if player_collision {
                println!("before {}", &game_data.score);
                game_data.score += 1;
                println!("after {}", &game_data.score);
            }
        }
    }
}
