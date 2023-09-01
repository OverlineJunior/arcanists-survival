use bevy::prelude::*;
use crate::components::{Player, Movement, Enemy};

#[derive(Bundle, Default)]
pub struct PlayerCharacter {
    pub player: Player,
    pub movement: Movement,
    pub sprite_bundle: SpriteBundle,
}

#[derive(Bundle, Default)]
pub struct EnemyCharacter {
    pub enemy: Enemy,
    pub movement: Movement,
    pub sprite_bundle: SpriteBundle,
}
