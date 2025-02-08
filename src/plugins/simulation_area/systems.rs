use bevy::prelude::*;

use crate::components::Agent;

use super::resources::SimulationArea;


pub fn check_agent_position(simulation_area: Res<SimulationArea>, mut agents: Query<&mut Transform, With<Agent>>) {

    for mut transform in &mut agents {
        transform.translation = transform.translation.clamp(simulation_area.0.min.extend(0.), simulation_area.0.max.extend(0.));
    }
}

pub fn remove_out_of_bounds_agents_on_creation(mut commands: Commands, simulation_area: Res<SimulationArea>, agents: Query<(Entity, &Transform), Added<Agent>>){
    for (entity, transform ) in &agents {

        if !simulation_area.0.contains(transform.translation.truncate()){
            commands.entity(entity).despawn();
        }
    }
}