use bevy::{ecs::system::Resource, math::Rect};

#[derive(Resource)]
pub struct SimulationArea(pub Rect);