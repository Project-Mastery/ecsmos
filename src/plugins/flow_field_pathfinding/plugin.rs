use bevy::prelude::*;

use crate::{Objective, Obstacle};

use super::{resources::{BlockedStatus, TargetStatus}, systems::*};

pub struct FlowFieldPathfindingPlugin;

impl Plugin for FlowFieldPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
        .add_systems(PreUpdate, create_colision_map::<BlockedStatus, Obstacle>)
        .add_systems(PreUpdate, create_colision_map::<TargetStatus, Objective>)
        .add_systems(PreUpdate, compute_proximity_map.after(create_colision_map::<BlockedStatus, Obstacle>).after(create_colision_map::<TargetStatus, Objective>))
        .add_systems(PreUpdate, create_vector_map.after(compute_proximity_map))
        // .add_systems(PreUpdate, create_target_map)
        //.add_systems(PostUpdate, draw_grid)
        //.add_systems(PostUpdate, draw_targets)
        .add_systems(PostUpdate, draw_vectors)
        //.add_systems(PostUpdate, draw_proximity)
        ;
    }
}

