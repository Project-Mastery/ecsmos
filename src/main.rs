mod components;
mod systems;
mod consts;
mod plugins;

use bevy::{
    color::palettes::tailwind::*,
    math::vec2,
    prelude::*,
};
use components::*;
use consts::*;


use plugins::{flow_field_pathfinding::plugin::FlowFieldPathfindingPlugin, simulation_area::plugin::SimulationAreaPlugin};

use systems::*;

fn main() {

    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
    .add_plugins((SimulationAreaPlugin{
        simulation_area: Rect::from_center_size(Vec2::ZERO * 361.415, 700. * Vec2::ONE)
    },))
    .add_plugins((FlowFieldPathfindingPlugin{ cell_size: 5.},))
    
        .add_systems(Startup, setup)
        // .add_systems(Startup, create_colision_map.after(setup))
        
        .add_systems(FixedUpdate, input_system)
        
        //.add_systems(FixedUpdate, motivation_force_system.before(apply_social_foces))
        .add_systems(FixedUpdate, obstacle_force.before(apply_social_foces))

        .add_systems(FixedUpdate, apply_social_foces.before(agent_max_speed_system))
        .add_systems(FixedUpdate, apply_repulsive_forces.before(agent_max_speed_system))
        //.add_systems(FixedUpdate, motivation_force_system.before(agent_max_speed_system))


        .add_systems(FixedUpdate, agent_max_speed_system.before(velocity_sytem))

        .add_systems(FixedUpdate, velocity_sytem.after(apply_social_foces))

        .add_systems(FixedUpdate, agent_araived_at_destination_system.after(velocity_sytem))
        // .add_systems(FixedUpdate, show_social_forces.after(apply_social_foces))
        ;
        

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());



    for x in (-500..-100).step_by(50) {
        for y in (-400..400).step_by(50) {
            commands.spawn((
                Agent,
                Speed(vec2(0., 0.)),
                ObstacleForce(vec2(0.,0.)),
                MotivationForce(vec2(0.,0.)),
                RepulsiveForce(vec2(0.,0.)),
                Mesh2d(meshes.add(Circle { radius: AGENT_RADIUS })),
                MeshMaterial2d(materials.add(Color::from(CYAN_500))),
                Transform::from_xyz(x as f32, y as f32, 0.1)
            ));
        }
    }

    commands.spawn((
        Objective,
        Shape::Circle(20.),
        Mesh2d(meshes.add(Circle { radius: 20.0 })),
        MeshMaterial2d(materials.add(Color::from(RED_500))),
        Transform::from_xyz(300.0, 0.0, 0.0),
    ));

    commands.spawn((
        Obstacle,
        Shape::Circle(50.),
        Mesh2d(meshes.add(Circle { radius: 50.0 })),
        MeshMaterial2d(materials.add(Color::from(GRAY_500))),
        Transform::from_xyz(100.0, 0.0, -0.5),
    ));
}


