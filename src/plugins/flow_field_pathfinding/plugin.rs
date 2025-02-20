use bevy::prelude::*;

use crate::{components::GridMap, plugins::simulation_area::resources::SimulationArea, systems::apply_social_foces, Objective, Obstacle};

use super::{models::{BlockedStatus, TargetProximity, TargetStatus}, resources::*, systems::*};

pub struct FlowFieldPathfindingPlugin{
    pub cell_size: f32
}

impl Plugin for FlowFieldPathfindingPlugin {
    fn build(&self, app: &mut App) {

        let cell_size = self.cell_size;

        app
        .insert_state(PathFindingOverlayState::ShowNone)
        .insert_state(ShowGridState::HideGrid);

        app.add_systems(Startup, move |simulation_area: Res<SimulationArea>, commands: Commands| {
            setup(simulation_area, commands, cell_size);
        })

        .add_systems(First, handle_grid_state_inputs)
        .add_systems(First, handle_overlay_inputs)
        
        .add_systems(PreUpdate, compute_colision_map::<BlockedStatus, Obstacle>)
        .add_systems(PreUpdate, compute_colision_map::<TargetStatus, Objective>)
        .add_systems(PreUpdate, compute_proximity_map.after(compute_colision_map::<BlockedStatus, Obstacle>).after(compute_colision_map::<TargetStatus, Objective>))
        .add_systems(PreUpdate, compute_vector_map.after(compute_proximity_map))
        
        .add_systems(Update, apply_objective_force_map.before(apply_social_foces))
        
        .add_systems(PostUpdate, draw_grid.run_if(in_state(ShowGridState::ShowGrid)))

        .add_systems(PostUpdate, draw_obstacles.run_if(in_state(PathFindingOverlayState::ShowObstacles)))
        .add_systems(PostUpdate, draw_targets.run_if(in_state(PathFindingOverlayState::ShowTargets)))
        .add_systems(PostUpdate, draw_proximity.run_if(in_state(PathFindingOverlayState::ShowProimity)))
        .add_systems(PostUpdate, draw_vectors.run_if(in_state(PathFindingOverlayState::ShowVectorField)));
    }
}

fn setup(simulation_area: Res<SimulationArea>, mut commands: Commands, cell_size: f32){

    let ratio: Vec2 = simulation_area.0.size() / cell_size * Vec2::ONE;
    let columns = ratio.x.ceil() as usize;
    let rows = ratio.y.ceil() as usize;

    commands.insert_resource(
        GridMap::new(
            columns, 
            rows, 
            simulation_area.0, 
            BlockedStatus::Empty
        )
    );

    commands.insert_resource(
        GridMap::new(
            columns, 
            rows, 
            simulation_area.0, 
            TargetStatus::NotTarget
        )
    );

    commands.insert_resource(
        GridMap::new(
            columns, 
            rows, 
            simulation_area.0, 
            TargetProximity::NotComputed
        )
    );

    commands.insert_resource(
        GridMap::new(
            columns, 
            rows, 
            simulation_area.0,
            Vec2::ZERO
        )
    );
}