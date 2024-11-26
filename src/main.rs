mod components;
mod systems;
mod consts;

use std::f32::consts::PI;

use bevy::{
    color::palettes::tailwind::*,
    math::vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use components::*;
use consts::*;
use systems::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, input_system)
        .add_systems(FixedPreUpdate, start_speed_system)
        .add_systems(FixedUpdate, motivation_force_system.before(agent_max_speed_system))
        .add_systems(FixedUpdate, obstacle_force.before(agent_max_speed_system))
        .add_systems(FixedUpdate, agent_max_speed_system.before(velocity_sytem))
        .add_systems(FixedUpdate, velocity_sytem.after(input_system))
        .add_systems(FixedUpdate, agent_araived_at_destination_system.after(velocity_sytem));

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for x in (-200..200).step_by(50) {
        commands.spawn((
            Agent,
            Speed(vec2(0., 0.)),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: AGENT_RADIUS })),
                material: materials.add(Color::from(CYAN_500)),
                transform: Transform::from_xyz(-300.0, x as f32, 0.1),
                ..default()
            },
        ));
    }

    

    commands.spawn((
        Agent,
        Speed(vec2(0., 0.)),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: AGENT_RADIUS })),
            material: materials.add(Color::from(CYAN_500)),
            transform: Transform::from_xyz(-300.0, -25.0, 0.1),
            ..default()
        },
    ));

    commands.spawn((
        Objective,
        Colider::Circle(20.),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 20.0 })),
            material: materials.add(Color::from(RED_500)),
            transform: Transform::from_xyz(300.0, 0.0, 0.0),
            ..default()
        },
    ));

    // let mut t = Transform::from_xyz(0.0, 0.0, -0.5);
    // t.rotate_z(1.0 / 4.0 * PI);

    commands.spawn((
        Obstacle,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
            material: materials.add(Color::from(GRAY_500)),
            transform: Transform::from_xyz(0.0, 0.0, -0.5),
            ..default()
        },
    ));

    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: Mesh2dHandle(meshes.add(Rectangle::new(1000.0, 700.0))),
    //     material: materials.add(Color::srgb(214., 219., 223.)),
    //     transform: Transform::from_xyz(0.0, 0.0, -1.0),
    //     ..default()
    // });
}


