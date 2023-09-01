use bevy::prelude::*;
use crate::{bundles::PlayerCharacter, components::Movement};

pub fn spawn_player(mut cmds: Commands, asset_server: Res<AssetServer>) {
   cmds.spawn(PlayerCharacter {
        movement: Movement {
            speed: 300.,
            ..default()
        },
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., 100.),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
