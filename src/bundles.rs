use bevy::prelude::*;
use crate::components::{Player, Movement};

#[derive(Bundle, Default)]
pub struct PlayerCharacter {
    pub player: Player,
    pub movement: Movement,
    pub sprite_bundle: SpriteBundle,
}
