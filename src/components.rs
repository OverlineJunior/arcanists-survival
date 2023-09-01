use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct Movement {
    pub direction: Vec2,
    pub speed: f32,
}
