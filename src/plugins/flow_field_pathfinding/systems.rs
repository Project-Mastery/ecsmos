use std::collections::VecDeque;

use bevy::{color::palettes::tailwind::{GREEN_500, PURPLE_500, RED_500}, math::VectorSpace, prelude::*, state::state};

use crate::{ components::{Agent, MotivationForce, Speed}, consts::{AGENT_DESIRED_SPEED, AGENT_MASS}, plugins::simulation_area::resources::SimulationArea, GridMap, Shape};

use super::{models::*, resources::{PathFindingOverlayState, ShowGridState}};

pub fn setup(simulation_area: Res<SimulationArea>, mut commands: Commands){

    let square_size = 50;

    commands.insert_resource(
        GridMap::new(
            square_size, 
            square_size, 
            simulation_area.0, 
            BlockedStatus::Empty
        )
    );

    commands.insert_resource(
        GridMap::new(
            square_size, 
            square_size, 
            simulation_area.0, 
            TargetStatus::NotTarget
        )
    );

    commands.insert_resource(
        GridMap::new(
            square_size, 
            square_size, 
            simulation_area.0, 
            TargetProximity::NotComputed
        )
    );

    commands.insert_resource(
        GridMap::new(
            square_size, 
            square_size, 
            simulation_area.0,
            Vec2::ZERO
        )
    );
}

pub fn create_colision_map<T, U>(mut map: ResMut<GridMap<T>>, targets: Query<(&Transform, &Shape), (With<U>, Changed<Transform>)> ) where T: CellStatus + 'static, U: Component{
    
    if !targets.is_empty(){
        map.reset(T::default());
    }
    
    for (transform, shape) in &targets {

        let Shape::Circle(radius) = shape;

        let center = transform.translation.truncate();
        let rect = shape.get_rectangle_with_center(center);

        let region = map.cells_within_rect(rect);
        
        let region = match region {
            Some(v) => v,
            None => continue,
        };

        for x in region.min.x..region.max.x {
            for y in region.min.y..region.max.y {

                
                let cell = IVec2::new(x, y);

                if let Some(value) = map.get_value_at_cell(cell){

                    if value == T::get_non_default_value(){
                        continue;
                    }
                }
                else {
                    continue;
                }

                let cell_center = map.get_coord(cell);

                if(cell_center - center).length() >= *radius {
                    let _ = map.set_value(cell, T::default());
                    continue;
                }
                
                let _ = map.set_value(cell, T::get_non_default_value());
            }
        }

        
    }
}

pub fn create_vector_map(mut vector_field: ResMut<GridMap<Vec2>>, proximity_map: ResMut<GridMap<TargetProximity>>){
    
    if !proximity_map.is_changed() {
        return;
    }
    
    for x_center in 0..proximity_map.columns{
        for y_center in 0..proximity_map.rows{

            let center: IVec2 = IVec2::new(x_center as i32, y_center as i32);
            let mut values = [Vec2::ZERO; 8];
            let mut i = 0;

            for dx in -1..=1{
                for dy in -1..=1{

                    if dx == 0 && dy == 0{
                        continue;
                    }

                    let delta = IVec2::new(dx, dy);
                    let current_pos = center + delta;
                    values[i] = match proximity_map.get_value_at_cell(current_pos) {
                        Some(TargetProximity::Computed(value)) => 1./value * delta.as_vec2(),
                        _ => Vec2::ZERO,
                    };
                
                    i += 1;
                }
            }

            let final_vector = values.iter().fold(Vec2::ZERO, |acc, &v| acc + v);
            let final_vector = final_vector.normalize() * AGENT_DESIRED_SPEED;

            vector_field.set_value(center, final_vector).ok();
        }
    }
    
}

pub fn compute_proximity_map(mut proximity_map: ResMut<GridMap<TargetProximity>>, obstacles_map: Res<GridMap<BlockedStatus>>, target_map: Res<GridMap<TargetStatus>>){
    
    if !obstacles_map.is_changed() && !target_map.is_changed(){
        return;
    }

    let mut open_list = VecDeque::new();

    proximity_map.reset(TargetProximity::NotComputed);

    for x in 0..proximity_map.columns {
        for y in 0..proximity_map.rows {
            let pos: IVec2 = IVec2::new(x as i32, y as i32);

            let proximity = match (obstacles_map.get_value_at_cell(pos), target_map.get_value_at_cell(pos)) {
                (Some(BlockedStatus::Blocked), _) => TargetProximity::Unreachable,
                (_, Some(TargetStatus::IsTarget)) => TargetProximity::Computed(0.),
                (_, _) => TargetProximity::NotComputed
            };

            if let TargetProximity::Computed(_) = proximity{
                open_list.push_back(pos);
            }

            proximity_map.set_value(pos, proximity).ok();
        }
    }

    while let Some(pivot_pos) = open_list.pop_front(){
        let value_pivot_pos =  proximity_map.get_value_at_cell(pivot_pos);

        let value_pivot_pos = match value_pivot_pos {
            None | Some(TargetProximity::Unreachable | TargetProximity::NotComputed)=> continue,
            Some(TargetProximity::Computed(value)) => value,
        };

        for x in -1..=1{
            for y in -1..=1{

                if x == 0 && y == 0{
                    continue;
                }

                let current_cell = pivot_pos + IVec2::new(x, y);

                let delta = match (x, y) {
                    (0, _) | (_, 0) => 1.,
                    (_, _) => 2_f32.sqrt(),
                };

                let value_at_cell = proximity_map.get_value_at_cell(pivot_pos + IVec2::new(x, y));

                match value_at_cell {
                    None | Some(TargetProximity::Unreachable)=> {},
                    Some(TargetProximity::NotComputed) => {
                        proximity_map.set_value(current_cell, TargetProximity::Computed(value_pivot_pos + delta)).unwrap();
                        open_list.push_back(current_cell);
                    },
                    Some(TargetProximity::Computed(value)) => {
                        let new_distance = value_pivot_pos + delta;
                        let distance = f32::min(value, new_distance);
                        proximity_map.set_value(current_cell, TargetProximity::Computed(distance)).unwrap();
                    }  
                };
            }
        }
    }
}

pub fn apply_vector_map(vector_field: ResMut<GridMap<Vec2>>, mut agents: Query<(&mut MotivationForce, &Transform, &Speed), With<Agent>>){
    
    for (mut motivation_force, transform, agent_speed) in &mut agents {

        let pos = transform.translation.truncate();

        let base_vector = match vector_field.get_value_at(pos){
            Some(value) => value,
            None => continue,
        };

        if base_vector.is_nan(){
            continue;
        }
        
        let final_force = base_vector - agent_speed.0;

        motivation_force.0 = final_force;
    }
}


pub fn handle_grid_state_inputs(grid_state: Res<State<ShowGridState>>, mut nex_grid_state: ResMut<NextState<ShowGridState>>, keys: Res<ButtonInput<KeyCode>>) {
    
    if keys.just_pressed(KeyCode::KeyG) {
        match grid_state.get() {
            ShowGridState::HideGrid => nex_grid_state.set(ShowGridState::ShowGrid),
            ShowGridState::ShowGrid => nex_grid_state.set(ShowGridState::HideGrid),
        }
    }
}

pub fn handle_overlay_inputs(state: Res<State<PathFindingOverlayState>>, mut next_state: ResMut<NextState<PathFindingOverlayState>>, keys: Res<ButtonInput<KeyCode>>) {
    
    let mut next = Option::None;

    if keys.just_pressed(KeyCode::KeyO) {
        next = Some(PathFindingOverlayState::ShowObstacles);
    }

    if keys.just_pressed(KeyCode::KeyP) {
        next = Some(PathFindingOverlayState::ShowProimity);
    }

    if keys.just_pressed(KeyCode::KeyT) {
        next = Some(PathFindingOverlayState::ShowTargets);
    }

    if keys.just_pressed(KeyCode::KeyV) {
        next = Some(PathFindingOverlayState::ShowVectorField);
    }

    if let Some(new_value) = next{
        if new_value == *state.get(){
            next_state.set(PathFindingOverlayState::ShowNone);
        } else {
            next_state.set(new_value);
        }
    }
}


pub fn draw_grid(mut gizmos: Gizmos, map: Res<GridMap<BlockedStatus>>){
    
    gizmos
        .grid_2d(
            map.area.center(),
            UVec2::new(map.columns as u32, map.rows as u32),
            map.cell_dimentions,
            LinearRgba::gray(0.05),
        )
        .outer_edges();


}

pub fn draw_obstacles(mut gizmos: Gizmos, map: Res<GridMap<BlockedStatus>>){

    let global_offset = Vec2::new(map.columns as f32, map.rows as f32) / 2.;
    let color = Color::from(RED_500);

    for x in 0..map.columns {
        for y in 0..map.rows {
            
            if let Some(BlockedStatus::Empty) = map.get_value_at_cell(IVec2::new(x as i32, y as i32)){
                continue;
            }

        
            let cell_top_left = map.area.center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.cell_dimentions;

            gizmos.line_2d(cell_top_left, cell_top_left + map.cell_dimentions, color);
            gizmos.line_2d(cell_top_left + map.cell_dimentions.with_x(0.), cell_top_left + map.cell_dimentions.with_y(0.), color);
        }
    }

}

pub fn draw_targets(mut gizmos: Gizmos, map: Res<GridMap<TargetStatus>>){

    let global_offset = Vec2::new(map.columns as f32, map.rows as f32) / 2.;
    let color = Color::from(GREEN_500);

    for x in 0..map.columns {
        for y in 0..map.rows {
            
            if let Some(TargetStatus::NotTarget) = map.get_value_at_cell(IVec2::new(x as i32, y as i32)){
                continue;
            }

        
            let cell_top_left = map.area.center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.cell_dimentions;

            gizmos.line_2d(cell_top_left, cell_top_left + map.cell_dimentions, color);
            gizmos.line_2d(cell_top_left + map.cell_dimentions.with_x(0.), cell_top_left + map.cell_dimentions.with_y(0.), color);
        }
    }

}

pub fn draw_proximity(mut gizmos: Gizmos, map: Res<GridMap<TargetProximity>>){

    let global_offset = Vec2::new(map.columns as f32, map.rows as f32) / 2.;
    

    for x in 0..map.columns {
        for y in 0..map.rows {
            
            if let Some(TargetProximity::Unreachable) = map.get_value_at_cell(IVec2::new(x as i32, y as i32)){
                continue;
            }

            let color = match map.get_value_at_cell(IVec2::new(x as i32, y as i32)) {
                None | Some(TargetProximity::Unreachable) => continue,
                Some(TargetProximity::NotComputed) => Color::from(PURPLE_500),
                Some(TargetProximity::Computed(value)) => Color::from(GREEN_500).with_alpha(1./(value + 1.)),
            };
        
            let cell_top_left = map.area.center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.cell_dimentions;

            gizmos.line_2d(cell_top_left, cell_top_left + map.cell_dimentions, color);
            gizmos.line_2d(cell_top_left + map.cell_dimentions.with_x(0.), cell_top_left + map.cell_dimentions.with_y(0.), color);
        }
    }

}

pub fn draw_vectors(mut gizmos: Gizmos, map: Res<GridMap<Vec2>>){

    let global_offset = Vec2::new(map.columns as f32, map.rows as f32) / 2.;
    

    for x in 0..map.columns {
        for y in 0..map.rows {
            
            if let Some(value) = map.get_value_at_cell(IVec2::new(x as i32,  y as i32)){
                
                let cell_top_left = map.area.center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.cell_dimentions;
                let cell_center = cell_top_left + Vec2::new(0.5, 0.5) * map.cell_dimentions;
                gizmos.arrow_2d(cell_center, cell_center + value * AGENT_MASS / 10., PURPLE_500);

            }
        
            
        }
    }

}