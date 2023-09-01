use bevy::prelude::*;
use crate::components::{Enemy, Player, Movement};

pub fn enemy_chase_player(
    mut enemy_query: Query<(&mut Movement, &Transform), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let plr_pos = player_query.single().translation;

    for (mut movement, transform) in &mut enemy_query {
        let dir = transform.looking_at(plr_pos, Vec3::Y).forward();
        println!("{:?}", dir);
        movement.direction = Vec2::new(dir.x, dir.y);
    }
}
