use bevy::prelude::*;

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Speed(pub Vec2);

#[derive(Component)]
pub struct Objective;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub enum Colider {
    Circle(f32),
    Rectangle{
        width: f32,
        height:f32
    }
}

#[derive(Resource)]
pub struct MaxAgentSpeed(pub f32);

