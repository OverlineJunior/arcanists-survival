pub mod systems;
pub mod components;
pub mod bundles;

use bevy::prelude::*;
use systems::{
    setup::setup,
    movement_input::movement_input,
    player_movement::player_movement,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (movement_input, player_movement))
        .run();
}
