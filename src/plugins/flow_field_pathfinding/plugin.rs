use bevy::prelude::*;

use crate::{systems::apply_social_foces, Objective, Obstacle};

use super::{models::{BlockedStatus, TargetStatus}, resources::*, systems::*};

pub struct FlowFieldPathfindingPlugin;

impl Plugin for FlowFieldPathfindingPlugin {
    fn build(&self, app: &mut App) {

        app.insert_state(PathFindingOverlayState::ShowNone)
        .insert_state(ShowGridState::HideGrid);

        app.add_systems(Startup, setup)
        .add_systems(First, handle_grid_state_inputs)
        .add_systems(First, handle_overlay_inputs)
        
        .add_systems(PreUpdate, create_colision_map::<BlockedStatus, Obstacle>)
        .add_systems(PreUpdate, create_colision_map::<TargetStatus, Objective>)
        .add_systems(PreUpdate, compute_proximity_map.after(create_colision_map::<BlockedStatus, Obstacle>).after(create_colision_map::<TargetStatus, Objective>))
        .add_systems(PreUpdate, create_vector_map.after(compute_proximity_map))
        
        .add_systems(FixedUpdate, apply_vector_map.before(apply_social_foces))
        
        .add_systems(PostUpdate, draw_grid.run_if(in_state(ShowGridState::ShowGrid)))

        .add_systems(PostUpdate, draw_obstacles.run_if(in_state(PathFindingOverlayState::ShowObstacles)))
        .add_systems(PostUpdate, draw_targets.run_if(in_state(PathFindingOverlayState::ShowTargets)))
        .add_systems(PostUpdate, draw_proximity.run_if(in_state(PathFindingOverlayState::ShowProimity)))
        .add_systems(PostUpdate, draw_vectors.run_if(in_state(PathFindingOverlayState::ShowVectorField)));
    }
}

