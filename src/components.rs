use anyhow::Result;

use bevy::prelude::*;



#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Speed(pub Vec2);

#[derive(Component)]
pub struct MotivationForce(pub Vec2);

#[derive(Component)]
pub struct ObstacleForce(pub Vec2);

#[derive(Component)]
pub struct RepulsiveForce(pub Vec2);

#[derive(Component)]
pub struct Objective;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub enum Shape {
    Circle(f32),
}

impl Shape {
    pub fn get_rectangle_with_center(&self, center: Vec2) -> Rect{
        match self {
            Shape::Circle(r) => Rect::from_center_half_size(center, Vec2::new(*r, *r)),
        }
    }
}

#[derive(Resource)]
pub struct GridMap<T> where T: Clone + Copy{

    grid: Vec<T>,
    pub cell_dimentions: Vec2,
    pub columns: usize,
    pub rows:usize,
    pub area: Rect,

}

impl<T> GridMap<T> where T: Clone + Copy{
    pub fn new(columns: usize, rows: usize, area: Rect, default_value: T) -> Self {
        
        let lengths = (area.max - area.min).abs();

        let cell_length = lengths.x / columns as f32;
        let cell_height = lengths.y / rows as f32;

        let cell_dimentions = Vec2::new(cell_length, cell_height);

        let total_cels = columns * rows;

        let grid = vec![default_value; total_cels];

        Self { 
            grid,
            cell_dimentions,
            columns,
            rows,
            area
        }
    }

    pub fn reset(&mut self, default_value: T){
        for i in 0..self.grid.len(){
            self.grid[i] = default_value;
        }
    }
    pub fn get_cell(&self, pos: Vec2) -> Option<IVec2>{

        let coords = self.get_cell_unsafe(pos);

        self.check_bounds(IVec2::new(coords.x, coords.y))
    }

    pub fn get_coord(&self, cell: IVec2) -> Vec2{
        let mim_coord = (cell.as_vec2() - Vec2::new(self.columns as f32, self.rows as f32) / 2.) * self.cell_dimentions * 2.;
        
        let half_cell_offset = self.cell_dimentions / 2.;

        self.area.center() + mim_coord / 2. + half_cell_offset
    }

    pub fn get_value_at_cell(&self, pos: IVec2) -> Option<T>{

        if let Some(pos) = self.check_bounds(pos){
            return Some(self.grid[pos.x as usize + pos.y as usize * self.columns]);
        }

        None
    }

    pub fn get_value_at(&self, pos: Vec2) -> Option<T>{
        self.get_value_at_cell(self.get_cell(pos)?)
    }

    pub fn set_value(&mut self, pos: IVec2, value: T) -> Result<(), ()>{
        
        if let Some(pos) = self.check_bounds(pos){
            self.grid[pos.x as usize + pos.y as usize * self.columns] = value;
            Result::Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_value_by_index(&self, i: usize) -> Option<T>{
        if i >= self.columns * self.rows {
            return None;
        }

        Some(self.grid[i])
    }

    pub fn set_value_by_index(&mut self, i: usize, value: T) -> Result<(), ()>{
        if i >= self.columns * self.rows {
            return Err(());
        }

        self.grid[i] = value;
        return Result::Ok(())
    }

    pub fn cells_within_rect(&self, search_area: Rect) -> Option<IRect>{

        let search_center = self.get_cell_unsafe(search_area.center());
        
        let size = (search_area.size() / (self.cell_dimentions))
        .ceil()
        .as_ivec2()
        .clamp(IVec2::ZERO, IVec2::new(self.columns as i32, self.rows as i32));

        if size.length_squared() == 0{
            return None;
        }

        Some(IRect::from_center_size(search_center, size + IVec2::ONE))
    }

    fn get_cell_unsafe(&self, pos: Vec2) -> IVec2 {
        let relative_pos = pos - self.area.min;

        (relative_pos / self.cell_dimentions).floor().as_ivec2()
    }

    fn check_bounds(&self, pos: IVec2) -> Option<IVec2>{
        
        if pos.x < 0 || pos.x >= self.columns as i32|| pos.y < 0 || pos.y >= self.rows as i32 {
            return None;
        }

        Some(pos)
    }
}



