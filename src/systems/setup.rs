use bevy::prelude::*;
use crate::bundles::PlayerCharacter;

pub fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
   cmds.spawn(Camera2dBundle::default());
   
   cmds.spawn(PlayerCharacter {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("test-arcanist.png"),
            ..default()
        },
        ..default()
    });
}
