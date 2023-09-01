use bevy::prelude::*;
use crate::{bundles::{PlayerCharacter, EnemyCharacter}, components::Movement};

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

    cmds.spawn(EnemyCharacter {
        movement: Movement {
            speed: 250.,
            ..default()
        },
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("test-arcanist.png"),
            transform: Transform {
                translation: Vec3::new(0., 200., 0.),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
