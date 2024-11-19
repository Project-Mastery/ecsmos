use bevy::prelude::*;

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Speed(pub Vec2);

#[derive(Component)]
pub struct Objective;

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource)]
pub struct MaxAgentSpeed(pub f32);