use bevy::prelude::*;

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Speed(pub Vec2);

#[derive(Component)]
pub struct MotivationForce(pub Vec2);

#[derive(Component)]
pub struct ObstacleForce(pub Vec2);

#[derive(Component)]
pub struct Objective;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub enum Colider {
    Circle(f32),
}

