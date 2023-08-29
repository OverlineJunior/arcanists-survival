use bevy::prelude::*;
use crate::components::{Movement, Player};

pub fn movement_input(keyboard: Res<Input<KeyCode>>, mut movements: Query<&mut Movement, With<Player>>) {
    for mut movement in &mut movements {
        let mut dir = Vec2::ZERO;

        if keyboard.pressed(KeyCode::W) {
            dir.y += 1.;
        }
        
        if keyboard.pressed(KeyCode::S) {
            dir.y -= 1.;
        }
        
        if keyboard.pressed(KeyCode::D) {
            dir.x += 1.;
        }
        
        if keyboard.pressed(KeyCode::A) {
            dir.x -= 1.;
        }

        movement.direction = dir;
    }
}
