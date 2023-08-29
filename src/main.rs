pub mod systems;
pub mod components;
pub mod bundles;

use bevy::prelude::*;
use systems::setup::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
