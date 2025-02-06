use std::{cmp, collections::VecDeque};

use bevy::{color::palettes::tailwind::{GREEN_500, PURPLE_500, RED_500}, math::{vec2, VectorSpace}, prelude::*, scene::ron::value};

use crate::{Agent, GridMap, Objective, Obstacle, Shape};

use super::resources::{BlockedStatus, CellStatus, TargetProximity, TargetStatus};

pub fn setup(mut commands: Commands){

    let square_size = 110;
    let square_length = 1100.;

    let x = GridMap::new(
        square_size, 
        square_size, 
        Rect::from_center_size(Vec2::ZERO * 361.415, square_length * Vec2::ONE), 
        BlockedStatus::Empty
    );

    let mut y = GridMap::new(
        square_size, 
        square_size, 
        Rect::from_center_size(Vec2::ZERO * 361.415, square_length * Vec2::ONE), 
        TargetStatus::NotTarget
    );

    commands.insert_resource(
        x
    );

    commands.insert_resource(
        y
    );

    commands.insert_resource(
        GridMap::new(
            square_size, 
            square_size, 
            Rect::from_center_size(Vec2::ZERO * 361.415, square_length * Vec2::ONE), 
            TargetProximity::NotComputed
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

pub fn draw_grid(mut gizmos: Gizmos, map: Res<GridMap<BlockedStatus>>){
    
    gizmos
        .grid_2d(
            map.area.center(),
            0.,
            UVec2::new(map.columns as u32, map.rows as u32),
            map.cell_dimentions,
            LinearRgba::gray(0.05),
        )
        .outer_edges();


    let global_offset = (Vec2::new(map.columns as f32, map.rows as f32) / 2.).floor();
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
    
    gizmos
        .grid_2d(
            map.area.center(),
            0.,
            UVec2::new(map.columns as u32, map.rows as u32),
            map.cell_dimentions,
            LinearRgba::gray(0.05),
        )
        .outer_edges();


    let global_offset = (Vec2::new(map.columns as f32, map.rows as f32) / 2.).floor();
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
        println!("Open list size = {:?}", open_list.len());
        let value_pivot_pos =  proximity_map.get_value_at_cell(pivot_pos);

        println!("{:?}", value_pivot_pos);

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
                    None | Some(TargetProximity::Unreachable)=> {
                        println!("{:?} Not computed or unrechable", current_cell);
                    },
                    Some(TargetProximity::NotComputed) => {
                        println!("{:?} Set first time value value = {}", current_cell, value_pivot_pos + delta);
                        proximity_map.set_value(current_cell, TargetProximity::Computed(value_pivot_pos + delta)).unwrap();
                        open_list.push_back(current_cell);
                    },
                    Some(TargetProximity::Computed(value)) => {
                        let new_distance = value_pivot_pos + delta;
                        let distance = f32::min(value, new_distance);
                        println!("{:?} Already computed, old value = {}, compare value = {}, new value = {}", current_cell, value, new_distance, distance);
                        proximity_map.set_value(current_cell, TargetProximity::Computed(distance)).unwrap();
                    }  
                };
            }
        }
    }
}

pub fn draw_proximity(mut gizmos: Gizmos, map: Res<GridMap<TargetProximity>>){
    
    gizmos
        .grid_2d(
            map.area.center(),
            0.,
            UVec2::new(map.columns as u32, map.rows as u32),
            map.cell_dimentions,
            LinearRgba::gray(0.05),
        )
        .outer_edges();


    let global_offset = (Vec2::new(map.columns as f32, map.rows as f32) / 2.).floor();
    

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