pub mod systems;
pub mod components;
pub mod bundles;

use bevy::prelude::*;
use systems::{
    setup::{
        spawn_camera::spawn_camera,
        spawn_player::spawn_player,
        spawn_enemy::spawn_enemy,
    },
    movement_input::movement_input,
    player_movement::player_movement,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_enemy))
        .add_systems(Update, (movement_input, player_movement))
        .run();
}
