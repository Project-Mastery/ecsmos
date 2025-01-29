use bevy::{color::palettes::tailwind::RED_500, math::{vec2, VectorSpace}, prelude::*, scene::ron::value};

use crate::{Agent, GridMap, Obstacle};

use super::resources::CellContents;

pub fn setup(mut commands: Commands){
    commands.insert_resource(
        GridMap::new(
            150, 
            150, 
            Rect::from_center_size(Vec2::ZERO * 361.415, 1100. * Vec2::ONE), 
            CellContents::Empty));
}

pub fn create_colision_map(mut map: ResMut<GridMap<CellContents>>, obstacles: Query<&Transform, (With<Obstacle>, Changed<Transform>)> ){
    
    if !obstacles.is_empty(){
        map.reset(CellContents::Empty);
    }
    
    for obstacle_transform in &obstacles {

        let obstacle_center = obstacle_transform.translation.truncate();
        let rect = Rect::from_center_half_size(obstacle_center, Vec2::new(50., 50.));

        let region = map.cells_within_rect(rect);
        
        let region = match region {
            Some(v) => v,
            None => continue,
        };

        for x in region.min.x..region.max.x {
            for y in region.min.y..region.max.y {

                
                let cell = IVec2::new(x, y);

                if let Some(value) = map.get_value_at_cell(cell){

                    if value == CellContents::Blocked{
                        continue;
                    }
                }
                else {
                    continue;
                }

                let cell_center = map.get_coord(cell);

                if(cell_center - obstacle_center).length() >= 50. {
                    let _ = map.set_value(cell, CellContents::Empty);
                    continue;
                }
                
                let _ = map.set_value(cell, CellContents::Blocked);
            }
        }

        
    }
}

pub fn draw_grid(mut gizmos: Gizmos, map: Res<GridMap<CellContents>>){
    
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
            
            if let Some(CellContents::Empty) = map.get_value_at_cell(IVec2::new(x as i32, y as i32)){
                continue;
            }

        
            let cell_top_left = map.area.center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.cell_dimentions;

            gizmos.line_2d(cell_top_left, cell_top_left + map.cell_dimentions, color);
            gizmos.line_2d(cell_top_left + map.cell_dimentions.with_x(0.), cell_top_left + map.cell_dimentions.with_y(0.), color);
        }
    }

}