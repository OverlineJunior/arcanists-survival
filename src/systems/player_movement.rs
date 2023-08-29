use bevy::prelude::*;
use crate::components::{Movement, Player};

pub fn player_movement(time: Res<Time>, mut transforms: Query<(&mut Transform, &Movement), With<Player>>) {
    for (mut transform, movement) in &mut transforms {
        transform.translation.x += movement.direction.x * movement.speed * time.delta_seconds();
        transform.translation.y += movement.direction.y * movement.speed * time.delta_seconds();
    }
}
