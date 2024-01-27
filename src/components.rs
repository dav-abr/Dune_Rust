use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Position {
    pub i: i8,
    pub j: i8,
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {}

#[derive(Component)]
pub struct GameCursor {}
