use bevy::prelude::*;

use super::systems::*;

pub struct FlowFieldPathfindingPlugin;

impl Plugin for FlowFieldPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
        .add_systems(PreUpdate, create_colision_map)
        .add_systems(PostUpdate, draw_grid);
    }
}

