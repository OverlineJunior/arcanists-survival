use bevy::prelude::*;
use crate::{bundles::PlayerCharacter, components::Movement};

pub fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
   cmds.spawn(Camera2dBundle::default());
   
   cmds.spawn(PlayerCharacter {
        movement: Movement {
            speed: 300.,
            ..default()
        },
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("test-arcanist.png"),
            ..default()
        },
        ..default()
    });
}
