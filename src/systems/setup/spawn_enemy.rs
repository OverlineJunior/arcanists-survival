use bevy::prelude::*;
use crate::{bundles::EnemyCharacter, components::Movement};

pub fn spawn_enemy(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn(EnemyCharacter {
        movement: Movement {
            speed: 125.,
            ..default()
        },
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("test-arcanist.png"),
            transform: Transform {
                translation: Vec3::new(0., 400., 0.),
                scale: Vec3::new(0.75, 0.75, 1.),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
