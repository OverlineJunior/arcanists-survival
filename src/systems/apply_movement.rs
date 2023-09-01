use bevy::prelude::*;
use crate::components::Movement;

pub fn apply_movement(time: Res<Time>, mut transforms: Query<(&mut Transform, &Movement)>) {
    for (mut transform, movement) in &mut transforms {
        transform.translation.x += movement.direction.x * movement.speed * time.delta_seconds();
        transform.translation.y += movement.direction.y * movement.speed * time.delta_seconds();
    }
}
