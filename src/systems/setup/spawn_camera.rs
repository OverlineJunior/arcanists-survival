use bevy::prelude::*;

pub fn spawn_camera(mut cmds: Commands) {
   cmds.spawn(Camera2dBundle::default());
}
