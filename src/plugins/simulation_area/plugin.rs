use bevy::{app::prelude::*, math::Rect};

use super::{resources::*, systems::*};

pub struct SimulationAreaPlugin{
    pub simulation_area: Rect
}

impl Plugin for SimulationAreaPlugin {
    fn build(&self, app: &mut App) {

        app.insert_resource(SimulationArea(self.simulation_area));

        app.add_systems(Update, check_agent_position)
        .add_systems(First, remove_out_of_bounds_agents_on_creation);
    }
}